# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 20
> **Topic No.:** 20
> **Topic Name:** Iterators & Higher-Order Programming
> **ประเด็นหลักที่ควรครอบคลุม:** iter(), map, filter, fold, collect, lazy evaluation

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายณัฐชนน รักวงศ์ | 670710646 | `@670710646` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวณัฐฐาพร เสตะวีระ | 670710647 | `@670710647` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฐดนัย ศรีไวย | 670710648 | `@670710648 ` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายพรรสวกร สมใจ | 670710651 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Iterators คือ interface ที่ใช้ดูข้อมูลใน collection/array ทีละตัว มีคุณสมบัติ lazy evaluation โดยจะไม่ทำงานจนกว่าจะต้องส่งผลลัพธ์ ช่วยประหยัดหน่วยความจำ ส่วน High-order programming ฟังก์ชันที่สามารถรับฟังก์ชันอื่นเป็น Argument หรือส่งคืนฟังก์ชันอื่นเป็นผลลัพธ์ได้ นิยมใช้กับเมธอดอย่าง map, filter, และ fold เพื่อประมวลผลข้อมูลใน Vector หรือ Iterator โดยไม่ต้องใช้ลูป for `

---

## 4. Key Concepts

### 4.1 `iter()`

**คำอธิบาย**

`สร้างตัววนซ้ำ (Iterator) โดยขอยืมอ่านข้อมูล โดยส่งคืนค่าเป็น Reference`

**ตัวอย่าง**

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("{}", number);
    }
}
```

**Explanation**

`numbers.iter() สร้าง Iterator ที่ขอยืมข้อมูลจาก numbers`

---

### 4.2 `map()`

`เปลี่ยนแปลง (Transform) ข้อมูลทุกตัวใน Iterator โดยส่งคืนค่าเป็น Iterator ตัวใหม่ ในแต่ละรอบ number จะเป็น Reference (&i32) ที่ชี้ไปยังข้อมูลเดิม ดังนั้นข้อมูลใน numbers ยังคงสามารถใช้งานต่อได้หลังจากวนลูปเสร็จ`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = numbers.iter().map(|number| number * 2);

    for number in doubled {
        println!("{}", number);
    }
}
```

**Explanation**

`map() นำข้อมูลแต่ละตัวใน Iterator ไปผ่าน Closure ที่กำหนด (|number| number * 2) หมายถึงนำแต่ละค่าไปคูณ 2 โดย map() จะคืนค่าเป็น Iterator ใหม่ และเป็น Lazy Iterator คือจะยังไม่ประมวลผลทันที จนกว่าจะมีการเรียกใช้งาน Iterator เช่น for loop`

---

### 4.3 `filter()`

`คัดกรองข้อมูลตามเงื่อนไข โดย Closure ต้องคืนค่าเป็น true หรือ false`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers = numbers
        .iter()
        .filter(|number| **number % 2 == 0);

    for number in even_numbers {
        println!("{}", number);
    }
}
```

**Explanation**

`filter() ตรวจสอบข้อมูลทีละตัวด้วยเงื่อนไข (**number % 2 == 0) ถ้าได้ true จะเก็บค่านั้นไว้ถ้าได้ false จะไม่ส่งค่านั้นต่อไป`

---

### 4.4 `fold()`

`ยุบรวมข้อมูลทั้งหมดใน Iterator ให้เหลือค่าเดี่ยว โดยคืนค่าผลลัพธ์สุดท้ายชนิดเดียว`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let sum = numbers
        .iter()
        .fold(0, |total, number| total + number);

    println!("{}", sum);
}
```

**Explanation**

`โดย fold() เริ่มต้นด้วยค่า 0 จากนั้นนำข้อมูลแต่ละตัวมาสะสมใน total (0 + 1 = 1, 1 + 2 = 3, 3 + 3 = 6, 6 + 4 = 10) ดังนั้นผลลัพธ์ = 10`

---

### 4.5 `collect()`

`รวบรวมข้อมูลจาก Iterator กลับไปเป็น Collection คืนค่าเป็น collection ใหม่ เช่น Vec, HashMap`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|number| number * 2)
        .collect();

    println!("{:?}", doubled);
}
```

**Explanation**

`map() สร้าง Iterator ใหม่จากข้อมูลเดิม เป็น (2, 4, 6) จากนั้น collect() จะดึงข้อมูลทั้งหมดจาก Iterator และรวบรวมกลับมาเป็น Vec<i32> ได้เป็น [2, 4, 6]`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `.iter()` | สร้าง Iterator ที่ยืมข้อมูล และส่งคืนค่าเป็น Reference | `numbers.iter()` |
| `.map(\|x\| ...)` | Transform ข้อมูลแต่ละตัว และคืนค่าเป็น Iterator ใหม่ | `.map(\|x\| x * 2)` |
| `.filter(\|x\| ...)` | คัดกรองข้อมูล โดยเก็บเฉพาะค่าที่เงื่อนไขเป็น `true` | `.filter(\|x\| x % 2 == 0)` |
| `.fold(init, \|acc, x\| ...)` | รวมข้อมูลทั้งหมดให้เหลือค่าเดียว โดยมีค่าเริ่มต้น | `.fold(0, \|sum, x\| sum + x)` |
| `.collect()` | รวบรวมข้อมูลจาก Iterator กลับเป็น Collection | `.collect::<Vec<_>>()` |
| `Iterator` เป็น Lazy | จะยังไม่ประมวลผลข้อมูลทันที จนกว่าเราจะเรียกใช้ Iterator เช่น `collect()`, `for`, หรือ `next()` | `.map(...).collect()` |
| `collect()` ต้องรู้ชนิดปลายทาง | Rust ต้องรู้ว่าต้องการสร้าง Collection ชนิดใด | `let v: Vec<_> = iter.collect()` |
| `Closure` | ฟังก์ชันแบบไม่ต้องตั้งชื่อที่สามารถรับค่าและใช้ตัวแปรจาก scope ภายนอกได้ | `let add = \|a, b\| a + b;` |
| `i32` | ชนิดข้อมูลจำนวนเต็ม (integer) แบบมีเครื่องหมาย (signed) ขนาด 32 บิต | `let x: i32 = 100;`| 

### Important Rules

1. `map()` และ `filter()` คืนค่าเป็น `Iterator` ใหม่ และยังไม่ประมวลผลทันที เพราะ Iterator ใน Rust เป็น Lazy
2. `iter()` จะยืมข้อมูล (`borrow`) ดังนั้นข้อมูลต้นฉบับยังสามารถใช้งานต่อได้
3. `collect()` ใช้ Consume Iterator และรวบรวมค่าที่ได้กลับเป็น Collection เช่น `Vec` หรือ `HashMap`
4. `fold()` จะ Consume Iterator และคืนค่าเป็นค่าเดี่ยว ไม่ใช่ Iterator
5. `map()` ใช้สำหรับ Transform ข้อมูล ส่วน `filter()` ใช้สำหรับคัดกรองข้อมูล และสามารถ Chain ต่อกันได้
6. `collect()` ต้องสามารถอนุมานได้ว่าต้องการ Collection ชนิดใด หาก Rust อนุมานไม่ได้ ต้องระบุ Type เช่น `Vec<_>`
7. Iterator สามารถ Chain หลาย operation ต่อกันได้ เช่น:
```rust
let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 5)
    .collect();
```

---


## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/670710647/rust-tutorial-2569/edit/main/20-iterators-higher-order-programming`

**Chapter Path:** `chapters/20-iterators-higher-order-programming/`

**Final PR:** `#[PR number]`

**Submitted by:** `Group 20`

**Date:** `[YYYY-MM-DD]`
