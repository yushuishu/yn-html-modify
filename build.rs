fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("D:\\itlike\\workSpace\\rust\\yn-html-modify\\yn-html-modify\\favicon.ico");
    // 添加元数据
    res.set("FileDescription", "Yank Note导出HTML文档进行简单修改：增加左边目录，GitHub：https://github.com/purocean/yn");
    res.set("CompanyName", "谁书-ss");
    res.set("ProductVersion", "1.0.0");
    res.set("LegalCopyright", "©谁书-ss");
    res.compile().unwrap();
}