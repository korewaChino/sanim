use proc_macro::{Group, Ident, TokenStream, TokenTree};
fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "พลาด" => "Err",
        "โอเค" => "Ok",
        "มาตรฐาน" => "std",
        "การจัดเก็บข้อมูล" => "collections",
        "สตริง" => "String",
        "สตริงระบบ" => "OsString",
        "เข้าออก" => "io",
        "โปรเจคต์" => "project",
        "กระบวนการ" => "process",
        "สตร" => "str",
        "แฮชแมพ" => "HashMap",
        "เวคเตอร์" => "Vec",
        "ปริยาย" => "Default",
        "ข้อผิดพลาด" => "Error",
        "เลือก" => "Option",
        "ดีบัก" => "Debug",
        "แสดง" => "Display",
        "โคลนนิ่ง" => "Clone",
        "โคลน" => "clone",
        "ค่า" => "Value",
        "ย่อย" => "derive",
        "มี" => "Some",
        "ว่าง" => "None",
        "ผลลัพธ์" => "Result",
        "คำสั่ง" => "Command",
        "ตนเอง" => "Self",
        "พิมพ์บรรทัด" => "println",
        "พิมพ์" => "print",
        "อ่าน" => "read",
        "เขียน" => "write",
        "เปิด" => "open",
        "ปิด" => "close",
        "อ่านเป็นสตริง" => "read_to_string",
        "เป็นสตริง" => "to_string",
        "เป็นสตร" => "as_str",
        "เก็บ" => "collect",
        "เก็บเป็น" => "collect",
        "ยกเลิก" => "break",
        "ไม่พร้อมกัน" => "async",
        "รอ" => "await",
        "วน" => "loop",
        "ย้าย" => "move",
        "ลัง" => "crate",
        "โค้ดเข้าถึงไม่ได้" => "unreachable_code",
        "ค่อยทำ" => "todo",
        "เป็น" => "as",
        "ค่าคงที่" => "const",
        "ลักษณะ" => "trait",
        "เสี่ยง" => "unsafe",
        "ใน" => "in",
        "จาก" => "from",
        "พลวัต" => "dyn",
        "แกะ" => "unwrap",
        "โดยปริยาย" => "default",
        "เป็นอ้างอิง" => "as_ref",
        "ภายนอก" => "extern",
        "เท็จ" => "false",
        "ฟังก์ชัน" => "fn",
        "เหนือ" => "super",
        "แทรก" => "insert",
        "นำ" => "get",
        "อนุญาต" => "allow",
        "ปัญหา" | "ชิบหาย" | "ไอ้เหี้ย" | "กรรม" | "อ้าว" => "panic",
        "โมดุล" => "mod",
        "แปรผัน" => "mut",
        "ใหม่" => "new",
        "สำหรับ" => "for",
        "เอาหรือใส่ด้วย" => "get_or_insert_with",
        "หลัก" => "main",
        "สาธารณะ" | "สธณ" => "pub",
        "ว่างเปล่า" => None?,
        "ให้ผล" => "return",
        "วิธี" => "impl",
        "อ้าง" => "ref",
        "จับคู่" => "match",
        "ถ้า" => "if",
        "ไม่งั้น" => "else",
        "ตน" => "self",
        "ให้" => "let",
        "สถิต" => "static",
        "ชุดข้อมูล" => "struct",
        "หวัง" => "expect",
        "เมื่อ" => "where",
        "ในเมื่อ" => "while",
        "ใช้" => "use",
        "นำเป็น" => "into",
        "จริง" => "true",
        "แจกแจง" => "enum",
        "เอา" => "take",
        "เอาออก" => "remove",
        "จำนวนเต็ม" => "i32",
        "จำนวนเต็มไม่ลบ" => "u32",
        "ทศนิยม" => "f32",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn sanim(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
