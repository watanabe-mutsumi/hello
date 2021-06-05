type Sid = usize; //学生ID
type Uid = usize; //大学ID

#[derive(Debug,Clone)]
pub(crate) struct Student{
    id: Sid,
    u_vec: Vec<Univ>,
}

impl Student {
    pub fn new(i: Sid) -> Self{
        Self{id: i, u_vec: Vec::new()}
    }
}

#[derive(Debug,Clone)]
pub(crate) struct Univ{
    id: Uid,
    s_vec: Vec<Sid>
}

impl Univ {
   pub fn new(i: Uid) -> Self{
       Self {id: i, s_vec: Vec::new()}
   } 
}

fn entry(us: &mut Vec<Univ>, e: (Uid, Sid)){
    let (uid, sid) = e;
    us[uid].s_vec.push(sid);
}

pub fn main() {
    let _students: Vec<Student> = (0..10).into_iter()
        .map(|s| Student::new(s))
        .collect();

    let mut univs: Vec<Univ> = (0..5).into_iter()
        .map(|u| Univ::new(u))
        .collect();

    let entires: Vec<(Uid, Sid)> = vec![
        (0,1), (1,3), (0,2), (3,5)
    ];

    entires.into_iter()
        .for_each(|(u,s)| univs[u].s_vec.push(s));
        // .for_each(|x| entry(&mut univs, x));

    // println!("Hello, world!");
    // println!("日本語");
    // println!("Studets: {:?}", students);
    println!("Univs: {:?}", univs);
}
