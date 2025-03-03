// use todo_cli::*;
use std::collections::HashMap;
use anyhow::{Result, Context};

#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo> {
        let content = std::fs::read_to_string("db.txt").context("打开失败!")?;

        let map = content.lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<_>>())
            .map(|v| (v[0].to_string(), v[1].parse::<bool>().unwrap()))
            .collect();

        // println!("map now is: {:?}", map);
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<()> {
        let mut content = String::new();

        for (key, val) in self.map {
            let rec = format!("{}\t{}\n", key, val);
            content.push_str(&rec);
        }
        std::fs::write("db.txt", content).context("持久化失败!")?;

        Ok(())
    }

    fn complete(&mut self, key: String) -> Option<()> {
        match self.map.get_mut(&key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() -> Result<()> {
    let action = std::env::args().nth(1).unwrap();

    let mut todo = Todo::new().context("初始化失败!")?;
    // println!("todo now is: {:?}", todo);

    match action.as_str() {
        "add" => {
            if let Some(item) = std::env::args().nth(2) {
                todo.insert(item);
                
                match todo.save() {
                    Ok(_) => println!("添加成功!"),
                    Err(e) => println!("添加失败! {}", e),
                }
            } else {
                println!("请输入待办事项!");
                return Ok(());
            }
            
        },
        "complete" => {
            if let Some(item) = std::env::args().nth(2) {
                match todo.complete(item.clone()) {
                    Some(_) => println!("{} 已完成!", item),
                    None => println!("{} 不存在!", item),
                }
    
                match todo.save() {
                    Ok(_) => println!("更新数据库成功!"),
                    Err(e) => println!("更新数据库失败! {}", e),
                }
            } else {
                println!("请输入待办事项!");
                return Ok(());
            }
        }
        "list" => {
            for (key, val) in todo.map {
                println!("{} {}", key, if val { "未完成" } else { "已完成" });
            }
        },
        _ => println!("不支持的操作!"),

    }

    Ok(())
}
