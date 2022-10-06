# sanim (สนิม)

Aren't you เหนื่อย writing Rust programs in English? Do you like saying "เหี้ย" a lot? Would you like to try something different, in an exotic and funny-sounding language? Would you want to bring some Thai touch to your programs?

**sanim** (Thai for Rust) is here to save your day! as it allows you to write Rust programs in Thai, using Thai keywords, Thai function names, and Thai idioms. Even the great leader Plaek Pibulsonggram himself would be proud of you for writing in such a civilized language!

Here's an example on what can be achieved with Sanim:

## trait and impl (aka ลักษณะและวิธีการ)

```rust
#[ย่อย(ดีบัก)]
สาธารณะ แจกแจง เพศ {
    ชาย,
    หญิง,
}
#[ย่อย(ดีบัก)]
สธณ ชุดข้อมูล บุคคล {
    สธณ ชื่อ: สตริง, // shorthand for pub
    สธณ อายุ: จำนวนเต็ม,
    สธณ เพศ: เพศ,
}

ลักษณะ มนุษย์ {
    ฟังก์ชัน เดิน(&ตน) {
        พิมพ์!("กำลังเดิน");
    }
    ฟังก์ชัน พูด(&ตน) {
        พิมพ์!("สวัสดี");
    }
    ฟังก์ชัน เล่น(&ตน);
}

วิธี มนุษย์ สำหรับ บุคคล {
    ฟังก์ชัน เล่น(&ตน) {
        พิมพ์!("เล่นเกมส์");
    }
}
```

## Support for various Thai phrases
(Thai does not have regional swear words)

```rust
#[อนุญาต(โค้ดเข้าถึงไม่ได้)]
ฟังก์ชัน บัก(&ตน) {
    ปัญหา!("หาสตริงไม่เจอ"); // the proper way to say "panic!"
    อ้าว!("งงดิ"); // the polite way
    ชิบหาย!("ไรวะ"); // when you're doing this for the 1000th time
    ไอ้เหี้ย!("ควย"); // when you're using unsafe code and it causes a memory leak
    กรรม!("ไม่น่าเลย"); // when you managed to create a footgun for yourself
}
```

## Other examples
See the [examples](./examples/src/main.rs) to get a rough sense of the whole thing.

## การมีส่วนร่วม
First of all, Thank you for participating in this one big joke. The Thai Government will probably promote this on national news as a "user friendly" way to learn programming. Feel free to throw in a few identifiers here and there, and make a PR against the `หลัก` (Thai for `main`) branch.

## But ทำไม?
- The French did it, so why can't we?
- proc macros are very fun

## Other languages
- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)


## Acknowledgements
Big thanks to [rouille](https://github.com/bnjbvr/rouille) for making me realize this was actually possible somehow. I remember seeing ruggine somewhere but I couldn't remember where the create is.

## สัญญาอนุญาต

WTFPL 1.0, because the Author is Thai. Also translated into Thai.