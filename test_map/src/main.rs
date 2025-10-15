
use std::collections::HashMap;

fn test1() {
    println!("===test1==================");
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // 键和值的类型参数来说， 可以使用下划线占位， 而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name 和 field_value 所有权被移动到 map 中， 因此这里不能再使用
        for (k, v) in &map {
            println!("{}: {}", k, v);
        }

        // map.insert(&field_name, &field_value); // 如果这么引用传入
        // println!("{} {}", field_name, field_value);
    }
}

fn test2() {
    println!("===test2==================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if score.is_none() {
        println!("{}: not found", team_name);
    } else {
        println!("{}: {}", team_name, score.unwrap());
    }
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
}

fn test3() {
    println!("===test3==================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);
    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);
}

fn test4() {
    println!("===test4==================");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        print!("{} ", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("");
    println!("{:?}", map);
}

struct Company {
    department_employees: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Self {
            department_employees: HashMap::<String, Vec<String>>::new(),
            // department_employees: HashMap::new(),
        }
    }

    fn add_employee(&mut self, department: &str, employee: &str) {
        // println!("Adding employee {} to department {}", employee, department);
        let employees = self.department_employees.entry(department.to_string()).or_insert(vec![]);
        employees.push(employee.to_string());
    }
    
    fn list_all_employees(&self) {
        for (department, employees) in &self.department_employees {
            println!("Department: {}, Employees: {:?}", department, employees);
        }
    }

    fn list_employees(&self, department: &str) {
        // println!("Listing employees in department: {}", department);
        let employees = self.department_employees.get(department);
        if employees.is_none() {
            println!("Department {} not found", department);
        } else {
            println!("Employees in department {}: {:?}", department, employees.unwrap());
        }
    }
    
}

fn test5() {
    /*
        使用哈希 map 和 vector， 创建一个文本接口来允许用户向公司的部门中增加员工的名
        字。 例如， “Add Sally to Engineering” 或 “Add Amir to Sales”。 接着让用户获取一个部门
        的所有员工的列表， 或者公司每个部门的所有员工按照字典序排列的列表。
    */
    println!("===test5==================");
    let mut company = Company::new(); // department -> employees
    let input_vec = vec![
        String::from("Add Sally to Engineering"),
        String::from("Add Amir to Sales"),
        String::from("Add Bob to Engineering"),
        String::from("Add Jack to Engineering"),
        String::from("Add John to Sales"),
    ];
    for input in input_vec {
        let mut command = input.split_whitespace();
        let mut command_name = command.next().unwrap();
        let mut employee = command.next().unwrap();
        let mut to = command.next().unwrap();
        let mut department = command.next().unwrap();
        println!("command_name: {}, employee: {}, department: {}", command_name, employee, department);
        if command_name == "Add" {
            company.add_employee(department, employee);
        }
    }
    company.list_all_employees();
    company.list_employees("Engineering");
    company.list_employees("Sales");

}

fn main() {
    test1(); // 创建
    test2(); // 访问
    test3(); // 更新
    test4(); // 计数
    test5(); // 增加员工训练
}
