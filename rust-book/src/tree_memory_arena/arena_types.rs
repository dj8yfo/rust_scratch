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

//! HasId for Node.

use std::{
  collections::HashMap,
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
};

use super::{Arena, Node};

pub trait HasId {
  fn get_id(&self) -> usize;
}

// Type aliases for readability.
pub type NodeRef<T> = Arc<RwLock<Node<T>>>;
pub type WeakNodeRef<T> = Weak<RwLock<Node<T>>>;
pub type ReadGuarded<'a, T> = RwLockReadGuard<'a, T>;
pub type WriteGuarded<'a, T> = RwLockWriteGuard<'a, T>;
pub type ArenaMap<T> = HashMap<usize, NodeRef<T>>;
pub type FilterFn<T> = dyn Fn(usize, ReadGuarded<Node<T>>) -> bool;
pub type ResultUidList = Option<Vec<usize>>;
pub type ShreableArena<T> = Arc<RwLock<Arena<T>>>;
