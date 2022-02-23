/*
 Copyright 2022 Nazmul Idris

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

use super::arena::Arena;
use super::{Node, ReadGuarded, ResultUidList, ShreableArena, WalkerFn};
use std::fmt::Debug;
use std::marker::{Send, Sync};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
pub struct MTArena<T>
where
  T: 'static + Debug + Send + Sync + Clone,
{
  arena_arc: ShreableArena<T>,
}

impl<T> MTArena<T>
where
  T: 'static + Debug + Send + Sync + Clone,
{
  pub fn new() -> Self {
    MTArena {
      arena_arc: Arc::new(RwLock::new(Arena::new())),
    }
  }

  pub fn get_arena_arc(&self) -> ShreableArena<T> {
    self.arena_arc.clone()
  }

  /// `walker_fn` is a closure that captures variables. It is wrapped in an `Arc` to be able to
  /// clone that and share it across threads.
  /// More info:
  /// 1. SO thread: <https://stackoverflow.com/a/36213377/2085356>
  /// 2. Scoped threads: <https://docs.rs/crossbeam/0.3.0/crossbeam/struct.Scope.html>
  pub fn tree_walk_parallel(
    &self,
    node_id: usize,
    walker_fn: Arc<WalkerFn<T>>,
  ) -> JoinHandle<ResultUidList> {
    let arena_arc = self.get_arena_arc();
    let walker_fn_arc = walker_fn.clone();

    spawn(move || {
      let read_guard: ReadGuarded<Arena<T>> = arena_arc.read().unwrap();
      let return_value = read_guard.tree_walk_dfs(node_id);

      // While walking the tree, in a separate thread, call the `walker_fn` for each node.
      if let Some(result_list) = return_value.clone() {
        result_list.clone().into_iter().for_each(|uid| {
          let node_arc_opt = read_guard.get_node_arc(uid);
          if let Some(node_arc) = node_arc_opt {
            let node_ref: ReadGuarded<Node<T>> = node_arc.read().unwrap();
            walker_fn_arc(uid, node_ref.payload.clone());
          }
        });
      }

      return_value
    })
  }
}
