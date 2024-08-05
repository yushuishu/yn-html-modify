use std::env;
use std::fs;
use std::io::{self};
use nipper::{Document, Selection};

// 修改地方：
// 1、修改<style>标签内容，将 article.markdown-body 的内容 max-width: 1024px; 修改为max-width: 1224px;，并添加添加新样式，修改浏览器滑动条 ::-webkit-scrollbar {width: 4px;height: 4px;}::-webkit-scrollbar-track {background-color: rgba(73, 177, 245, .2);border-radius: 2em;}::-webkit-scrollbar-thumb {background-color: #49b1f5;background-image: -webkit-linear-gradient(45deg, hsla(0, 0%, 100%, .4) 25%, transparent 0, transparent 50%, hsla(0, 0%, 100%, .4) 0, hsla(0, 0%, 100%, .4) 75%, transparent 0, transparent);border-radius: 2em;}::-webkit-scrollbar-corner {background-color: transparent;}::-moz-selection {color: #fff;background-color: #49b1f5;}
// 2、修改<body>标签内容，将行内样式（需要判断是否存在） style="display: flex;overflow: hidden;height: 100vh" 修改为 style="display: flex;overflow: hidden;height: 96vh"
// 3、修改标签<article data-v-87564206="" class="markdown-body" style="font-size: 16px;" powered-by="Yank Note">...</div>行内样式，
//      设置行内样式：style="height: 100%;font-size: 16px;overflow-y: auto;position: relative;"
// 4、复制标签<div class="table-of-contents">...</div>内容，
//      添加到<body>标签中，与标签<article>标签同级，并在标签<article>前面，
//      设置行内样式：style="width: 300px;height: 100%;font-size: 16px;overflow-y: auto;position: relative;"
// 5、添加底部空行（导出html的文档，文档底部可能会看不到部分内容，通过添加换行符，显示正文内容）

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- [input_file] [output_file]");
        return Ok(());
    }
    let input_file = &args[1];
    let output_file = &args[2];
    println!("原始文件：{}", input_file);
    println!("输出文件：{}", output_file);
    let content = fs::read_to_string(input_file).unwrap_or_else(|error| {
        eprintln!("读取文件失败: {}", error);
        std::process::exit(1);
    });
    let document = Document::from(&content);


    // 1. 修改 <style> 标签内容，并添加新的 css 样式
    let new_style_tags_text = document.select("head").select("style").first().text()
            .replace("max-width: 1024px;", "max-width: 1224px;");

    let new_css = r#"
::-webkit-scrollbar {
    width: 4px;
    height: 4px;
}
::-webkit-scrollbar-track {
    background-color: rgba(73, 177, 245, .2);
    border-radius: 2em;
}
::-webkit-scrollbar-thumb {
    background-color: #49b1f5;
    background-image: -webkit-linear-gradient(45deg, hsla(0, 0%, 100%, .4) 25%, transparent 0, transparent 50%, hsla(0, 0%, 100%, .4) 0, hsla(0, 0%, 100%, .4) 75%, transparent 0, transparent);
    border-radius: 2em;
}
::-webkit-scrollbar-corner {
    background-color: transparent;
}
::-moz-selection {
    color: #fff;
    background-color: #49b1f5;
}
"#.to_string();

    let result_style_tags_text = format!("{}\n{}", new_css, new_style_tags_text);
    document.select("head").select("style")
            .set_html(result_style_tags_text);


    // 2. 修改 <body> 标签内容
    document.select("body").first()
            .set_attr("style", "display: flex;overflow: hidden;height: 96vh;");


    // 3. 修改标签<article data-v-87564206="" class="markdown-body" style="font-size: 16px;" powered-by="Yank Note">...</div>行内样式，
    //      设置行内样式：style="height: 100%;font-size: 16px;overflow-y: auto;position: relative;"
    document.select("body").select("article").first()
            .set_attr("style", "height: 100%;font-size: 16px;overflow-y: auto;position: relative;");


    // 4、复制标签<div class="table-of-contents">...</div>内容，
    //      添加到<body>标签中，与标签<article>标签同级，并在标签<article>前面，
    //      设置行内样式：style="width: 300px;height: 100%;font-size: 16px;overflow-y: auto;position: relative;"
    let table_contents_div = document.select("body").select("article").select("div.table-of-contents").first().html().to_string();
    document.select("body").first().prepend_html(table_contents_div);
    document.select("body").select("div.table-of-contents").first().set_attr("style", "width: 300px;height: 100%;font-size: 16px;overflow-y: auto;position: relative;");


    // 5、添加底部空行（导出html的文档，文档底部可能会看不到部分内容，通过添加换行符，显示正文内容）
    document.select("body").select("article").append_html("<br><br><br><br><br><br>");


    // 写回到输出文件
    fs::write(output_file, document.html().to_string())?;

    println!("处理完成");
    Ok(())
}


pub trait SelectionExt<'a> {
    fn prepend_selection(&mut self, sel: Selection);
    fn prepend_html(&mut self, html: String);
}

impl<'a> SelectionExt<'a> for Selection<'a> {
    fn prepend_selection(&mut self, sel: Selection) {
        // 创建一个新的 Vec<Node> 来存储新的子节点顺序
        let mut new_children = vec![];
        // // 将要插入的选择的所有节点添加到新的子节点顺序中
        for node in sel.nodes() {
            new_children.push(node.clone());
        }
        // 然后将当前的子节点添加到新的顺序中
        for child in self.children().nodes() {
            new_children.push(child.clone());
        }
        // 移除所有的子节点
        self.set_html("");
        // 将新的子节点顺序设置回去
        for child in new_children {
            // 不可以使用append_selection()方法添加
            self.append_html(Selection::from(child).html());
        }
    }

    fn prepend_html(&mut self, html: String)  {
        // 创建一个新的 Vec<Node> 克隆存储旧的子节点
        let mut new_children = vec![];
        for child in self.children().nodes() {
            new_children.push(child.clone());
        }

        // 移除所有的子节点
        self.set_html("");

        // 添加新节点
        self.set_html(html);

        // 将旧的子节点再设置回去
        for child in new_children {
            // 不可以使用append_selection()方法添加
            self.append_html(Selection::from(child).html());
        }
    }

}
