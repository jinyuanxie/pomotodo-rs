//Copyright 2017 KAMYUEN
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

extern crate pomotodo;
extern crate serde_json;

use pomotodo::session::Session;
use pomotodo::pomo::Pomo;
use pomotodo::todo::{Todo, SubTodo};

#[test]
fn test_session() {
    let sess = Session::with_token("Your token here").unwrap();

    let mut pomo = sess.create_pomo(&Pomo { ..Default::default() }).unwrap();
    let patched_pomo = sess.update_pomo(&pomo.uuid.unwrap(), "Test pomo patch".to_string())
        .unwrap();
    pomo.description = "Test pomo patch".to_string();

    assert_eq!(pomo.description, patched_pomo.description);

    assert!(match sess.delete_pomo(&pomo.uuid.unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    });

    let mut todo = sess.create_todo(&Todo { ..Default::default() }).unwrap();
    todo.description = "Test todo patch".to_string();
    let patched_todo = sess.update_todo(&todo).unwrap();
    assert_eq!(todo.description, patched_todo.description);

    let mut sub_todo = sess.create_subtodo(&todo.uuid.unwrap(), &SubTodo { ..Default::default() })
        .unwrap();
    sub_todo.description = "Test sub todo patch".to_string();
    let patched_sub_todo = sess.update_subtodo(&todo.uuid.unwrap(), &sub_todo).unwrap();

    assert_eq!(sub_todo.description, patched_sub_todo.description);

    assert!(match sess.delete_subtodo(&todo.uuid.unwrap(), &sub_todo.uuid.unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    });

    assert!(match sess.delete_todo(&todo.uuid.unwrap()) {
        Ok(_) => true,
        Err(_) => false,
    });
}
