use std::{path::PathBuf, fs::File, io::{Read}};

use anyhow::Ok;

pub fn parse_htmls(htmls: Vec<PathBuf>) -> anyhow::Result<Vec<PR>> {
    let mut prs = vec![];
    
    htmls.iter().for_each(|html_file| {
        let text = match read_html(html_file){
            anyhow::Result::Ok(s) => s,
            Err(err) => {
                eprintln!("Fail to open html cache {:?}, will be skipped.\n{:?}", html_file, err);
                return;
            }
        };

        prs.push(match PR::new(&text){
            anyhow::Result::Ok(pr) => pr,
            Err(err)=> {
                eprintln!("Fail to parse html {:?}, will be skipped.\n{:?}", html_file, err);
                return;
            }
        });
    });

    Ok(prs)
}

fn read_html(file: &PathBuf) -> anyhow::Result<String> {
    let mut fptr = File::open(file)?;

    let mut text = String::new();
    fptr.read_to_string(&mut text)?;

    Ok(text)
}

#[derive(Debug)]
pub struct PR {
    id: u32,
    modified_files: Vec<ModifiedFile>,
}

impl PR {
    pub fn new(text: &String) -> anyhow::Result<Self> {
        let mut id = 0;
        let mut modified_files = vec![];

        let mut iter = text.lines().into_iter();
        let mut line = iter.next();
        while line != None {
            let s = line.unwrap();
            
            // find pr id
            if s.contains("<title") {
                id = s[s.find('#').unwrap() + 1..s.find('#').unwrap() + s[s.find('#').unwrap()..].find(' ').unwrap()].parse::<u32>()?;
            }

            // find modifed file
            if s.contains("<span class=\"Truncate\">") {
                // find modified file-path
                let s = iter.next().unwrap();
                let file_path = &s[s.find("title=\"").unwrap() + "title=\"".len()..
                    s[s.find("title=\"").unwrap() + "title=\"".len()..].find("\"").unwrap() + s.find("title=\"").unwrap() + "title=\"".len()];
                println!("{:?}", file_path);

                // find modifed lines
                while line != None && line.unwrap().ne("<tbody>") {
                    line = iter.next();
                }


                
            }
            line = iter.next();
        }



        // for line in text.lines() {
        //     if line.contains("<title") {
        //         id = line[line.find('#').unwrap() + 1..line.find('#').unwrap() + line[line.find('#').unwrap()..].find(' ').unwrap()].parse::<u32>()?;
        //     }

        //     if line.contains("<span data-filterable-item-text hidden>") {
        //         let file_path = &line[line.find("<span data-filterable-item-text hidden>").unwrap()  + "<span data-filterable-item-text hidden>".len()..
        //             line.find("</span>").unwrap()];

        //         let file_path = PathBuf::from(file_path);
        //         println!("{:?}", PathBuf::from(file_path));
        //     }
        // }
        
        assert_ne!(id, 0);
        assert_ne!(modified_files.len(), 0);

        Ok(PR{
            id,
            modified_files,
        })
    }
}

#[derive(Debug)]
enum ModifyType {
    New,
    Del,
    Move,
    Edit,
}

#[derive(Debug)]
struct ModifiedFile {
    path: PathBuf,
    modify_type: ModifyType,
    modify_lines: Option<Vec<ModifiedLine>>,
}

#[derive(Debug)]
struct ModifiedLine {
    line_num: u32,
    old: String,
    new: String,
}



#[test]
fn parse_0(){
    let dom = read_html(&PathBuf::from("/home/fxl191220029/study/RollBackTool/cache/html/91743.html"));
    match dom {
        anyhow::Result::Ok(dom) => {
            println!("{:?}", PR::new(&dom));
        }
        Err(err) => eprintln!("{:?}", err),
    };
}
