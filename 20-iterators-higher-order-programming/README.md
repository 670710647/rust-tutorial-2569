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

### Example 1 — ระบบคัดเลือกนักศึกษารับทุน (Scholarship system)

**Purpose:** เพื่อสาธิตการประมวลผลข้อมูลด้วย Rust Iterator โดยใช้ filter() ในการคัดเลือกข้อมูล, map() ในการแปลงข้อมูล และ collect() ในการรวบรวมผลลัพธ์ พร้อมแสดงแนวคิดของ Lazy Evaluation ในการทำงานของ Iterator 

```rust
fn main() {
    let scores = vec![35, 50, 68, 72, 90, 45, 80];

    println!("Original Scores: {:?}", scores);

    let scholarship_students = scores // ยังไม่คำนวณ สร้าง pipeline ไว้ก่อน
        .iter()
        .filter(|score| **score >= 60)
        .map(|score| {
            println!("Adding bonus to: {}", score);
            score + 5
        });

    //จุดที่เริ่มประมวลผลจริง
    let final_scores: Vec<i32> = scholarship_students.collect();

    println!("\n========= Scholarship Award Results =========");

    for score in &final_scores {
        println!("Student recieved scholarship | Final score: {}", score);
    }

    println!("\nTotal Scholarship Students: {}", final_scores.len());

    println!("========= Congratuations ! =========");
}

```

**Expected Output**

```text
Original Scores: [35, 50, 68, 72, 90, 45, 80]
Adding bonus to: 68
Adding bonus to: 72
Adding bonus to: 90
Adding bonus to: 80

========= Scholarship Award Results =========
Student recieved scholarship | Final score: 73
Student recieved scholarship | Final score: 77
Student recieved scholarship | Final score: 95
Student recieved scholarship | Final score: 85

Total Scholarship Students: 4
========= Congratuations ! =========
```

**Explanation**

โปรแกรมเริ่มต้นจากการรับข้อมูลคะแนนของนักศึกษา จากนั้นสร้าง Iterator เพื่อเข้าถึงข้อมูลทีละรายการ แล้วทำการคัดเลือกเฉพาะนักศึกษาที่มีคะแนนตั้งแต่ 
คะแนนขึ้นไปด้วย filter() หลังจากนั้นใช้ map() เพื่อเพิ่มคะแนนโบนัสให้กับนักศึกษาที่่ผ่านเกณฑ์ ในขั้นตอนนี้ Rust จะยังไม่ประมวลผลข้อมูลจริง เนื่องจาก Iterator ใช้หลักการ Lazy Evaluation ซึ่งจะรอจนกว่าจะมีการร้องขอผลลัพธ์ เมื่อเรียกใช้ collect() โปรแกรมจึงเริ่มประมวลผลข้อมูลทั้งหมดและรวบรวมผลลัพธ์ออกมาเป็น Vector ใหม่ที่เก็บคะแนนของนักศึกษาที่ได้รับทุนหลังเพิ่มโบนัส

---

### Example 2 — ระบบสั่งอาหาร (Food Order System)

**Purpose:** เพื่อสาธิตการประมวลผลข้อมูลด้วย Rust Iterator โดยใช้ filter() ในการคัดเลือกเมนูที่มีราคาไม่เกิน 200 บาท, map() ในการคำนวณราคาหลังส่วนลด และ fold() ในการรวมราคาของเมนูที่ผ่านเงื่อนไขทั้งหมด พร้อมแสดงแนวคิด Higher-Order Programming ผ่านการใช้งาน Closure

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

โปรแกรมจำลองระบบสั่งอาหาร โดยมีรายการเมนูและราคาของแต่ละเมนู ลูกค้าต้องการเลือกเฉพาะเมนูที่มีราคาไม่เกิน 200 บาท

โปรแกรมจะใช้ filter() เพื่อคัดเลือกเฉพาะเมนูที่ตรงตามเงื่อนไข จากนั้นใช้ map() เพื่อคำนวณราคาหลังได้รับส่วนลด 10% และใช้ fold() เพื่อรวมราคาของเมนูทั้งหมดที่ผ่านเงื่อนไข เพื่อคำนวณยอดชำระทั้งหมด

ในแต่ละขั้นตอนจะมีการใช้ Closure เป็นฟังก์ชันสำหรับกำหนดวิธีการประมวลผลข้อมูล ทำให้ตัวอย่างนี้สามารถแสดงแนวคิด Higher-Order Programming ได้ด้วย
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

### 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

- **Method chaining:** ใช้เครื่องหมาย `.` ต่อ method จากซ้ายไปขวาตามลำดับการประมวลผล เช่น `.iter().filter(..).map(..).collect()`
- **Closure syntax:** เขียนด้วย `|พารามิเตอร์| นิพจน์` เช่น `|number| number * 2` หรือ `|number| { number * 2 }`
- **Pattern ใน parameter:** closure และ `for` รองรับ destructuring เช่น `|&number|` หรือ `|(index, value)|`
- **Type annotation สำหรับ `collect`:** ระบุชนิดข้อมูลด้วย `let v: Vec<_> = ...` (vec) หรือใช้ turbofish `collect::<Vec<_>>()`
- **`for` loop:** เป็น syntactic sugar ที่เรียก `IntoIterator::into_iter()` และวน `next()` ให้อัตโนมัติ

```rust
let doubled = vec![1, 2, 3];.iter().map(|number| number * 2).collect::<Vec<_>>();
```

### 9.2 Semantics

- **ความหมายของ `iter()`:** ไม่ได้ทำงานทันที เป็นเพียงการสร้าง iterator ที่ยืมข้อมูลมา
- **Adapter vs Consumer:** adapter (`map`, `filter`, `take`) เป็น lazy เพียงคืน iterator ตัวใหม่ ส่วน consumer (`collect`, `fold`, `sum`, `for`) คือตัวที่ดึงข้อมูลออกมาและทำให้ chain ประมวลผลจริง
- **Lazy evaluation:** adapter จะไม่ประมวลผลจนกว่าจะมี consumer เรียกใช้ ดูตัวอย่างลำดับการทำงาน:

```rust
let it = vec![1, 2, 3].iter().map(|number| {
    println!("processing {}", number);
    number * 2
});

println!("before collect");           // ยังไม่พิมพ์ processing
let out: Vec<i32> = it.collect();     // พิมพ์ processing 1, 2, 3
```

- **การประมวลผลทีละ element (pull-based):** เมื่อมี consumer, element แต่ละตัวจะผ่านทั้งสาย `filter -> map -> ...` ครบก่อนจึงไปตัวถัดไป
- **Short-circuit:** consumer อย่าง `find position` หยุดทันทีเมื่อได้คำตอบ
- **Iterator ใช้ครั้งเดียว:** consumer ส่วนใหญ่ เช่น `collect`, `fold`, `sum` รับ iterator ไปทั้งตัว (ownership) จึงเรียกใช้ iterator ตัวเดิมซ้ำไม่ได้
- **ความหมายของ `fold`:** `fold(init, |acc, number| ...)` คือการลดรูป (reduction) จากซ้ายไปขวา โดย `acc` คือค่าที่สะสมมาถึงตอนนี้

### 9.3 Type System

- **`Iterator` trait:** ประกอบด้วย associated type `Item` และเมธอด `next(&mut self) -> Option<Self::Item>` โดย `None` หมายถึงหมดข้อมูล

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- **Adapter เป็น struct ที่มี generic:** `map` คืนค่าเป็น `Map<I, F>` และ `filter` คืน `Filter<I, P>` ทำให้ compiler รู้ชนิดครบตอน compile time
- **Closure type:** closure แต่ละตัวมีชนิดเฉพาะตัวที่ compiler สร้างให้ และ implement trait `Fn`, `FnMut`, `FnOnce` ให้อัตโนมัติตามวิธีที่ใช้ตัวแปรที่จับมา (`Fn` เป็นกรณีที่เข้มที่สุดและใช้ได้ในบริบทของ `FnMut` และ `FnOnce` ด้วย)

| Trait | เงื่อนไขของ closure | เรียกได้กี่ครั้ง |
|-|-|-|
| `FnOnce` | ทุก closure เป็นได้ (อาจย้ายตัวแปรที่จับออกไปใช้) | 1 ครั้ง |
| `FnMut` | ไม่ย้ายตัวแปรที่จับออก แต่อาจแก้ไขได้ | หลายครั้ง |
| `Fn` | ไม่ย้ายและไม่แก้ไขตัวแปรที่จับ | หลายครั้ง |

- **Type inference:** compiler อนุมานชนิดของ parameter และ return ของ closure จากบริบทได้ ไม่ต้องเขียนชนิดเอง
- **`collect` ใช้ trait `FromIterator`:** ชนิดปลายทางเป็นตัวกำหนดผลลัพธ์ เช่น `Vec<T>`, `HashSet<T>`, `String` และยังรวมเป็น `Result<Vec<T>, E>` หรือ `Option<Vec<T>>` ได้
- **ชนิดของ `Item` ต่างกันตามวิธีสร้าง iterator:**

| เมธอด | `Item` | ความหมาย |
|-|-|-|
| `iter()` | `&T` | ยืมแบบอ่านอย่างเดียว |
| `iter_mut()` | `&mut T` | ยืมแบบแก้ไขได้ |
| `into_iter()` | `T` | ย้าย ownership ออกมา |

### 9.4 Memory / Resource Management

- **Ownership และ borrowing เกี่ยวข้องโดยตรง:** `iter()` ยืม collection ไว้ ระหว่างที่ iterator ยังถูกใช้งานอยู่จะแก้ไข collection ต้นทางไม่ได้ (borrow checker ป้องกัน) ส่วน `into_iter()` ย้าย ownership เข้าไปใน iterator
- **ไม่มี intermediate allocation:** เพราะเป็น lazy evaluation, method chaining `map`/`filter` จึงไม่สร้าง `Vec` ชั่วคราวระหว่างทาง ใช้หน่วยความจำ stack เป็นหลัก มี heap allocation เฉพาะตอน `collect` สร้าง collection ใหม่
- **Closure capture:** closure จับตัวแปรได้ 3 แบบ คือยืมแบบอ่าน, ยืมแบบแก้ไข หรือ `move` เพื่อย้าย ownership เข้าไป
- **Zero-cost abstraction:** compiler ทำ monomorphization และ inlining ทำให้ iterator chain ถูกแปลงเป็น loop ธรรมดา ประสิทธิภาพใกล้เคียงหรือเท่ากับเขียน `for` loop เอง
- **ไม่มี garbage collector:** ทรัพยากรถูกคืนอัตโนมัติผ่าน `Drop` เมื่อ iterator หรือ collection ออกนอก scope

### 9.5 Abstraction / Other PPL Concepts

- **Abstraction:** `Iterator` trait ซ่อนรายละเอียดว่าข้อมูลมาจากไหน (array, `Vec`, ไฟล์, ช่วงตัวเลข, network) ผู้ใช้เห็นเพียงอินเทอร์เฟซเดียวคือ `next()`
- **Paradigm:** ผสม **functional programming** (HOF, closure, immutability by default) เข้ากับ **imperative/systems programming** ในภาษาเดียว
- **Higher-order function:** ฟังก์ชันอย่าง `map` รับ closure เป็น argument และ function ถือเป็น first-class value
- **Generics และ trait bound:** ฟังก์ชันที่รับ closure เขียนด้วย bound เช่น `fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`
- **Composability:** ต่อ adapter หลายตัวเข้าด้วยกันเป็น pipeline ได้ และสร้าง iterator ของตัวเองได้โดย implement `Iterator`
- **Scope และ binding:** closure มี lexical scope จับตัวแปรจาก environment ที่สร้างมัน (lexical closure)
- **Infinite iterator:** เพราะ lazy evaluation จึงสร้างลำดับไม่จำกัดได้ เช่น `(1..).filter(..).take(5)`

### 9.6 Why Rust?

- **Safety:** borrow checker ป้องกัน iterator invalidation (การแก้ collection ระหว่างวน) ตั้งแต่ตอน compile ซึ่งเป็นบั๊กที่พบบ่อยใน C++
- **Reliability:** `Option` จาก `next()` บังคับให้จัดการกรณีข้อมูลหมด (None) และ `collect` รวม error ทำให้จัดการได้ทีเดียว
- **Performance:** zero-cost abstraction ทำให้เขียนโค้ดระดับสูงแบบ functional ได้โดยไม่เสียความเร็วเมื่อเทียบกับ loop เขียนมือ
- **Expressiveness:** โค้ดสั้น อ่านง่าย
- **Concurrency:** ต่อยอดเป็น parallel iterator ได้ง่าย (เช่น crate `rayon` ที่เปลี่ยน `iter()` เป็น `par_iter()`) โดยยังได้ความปลอดภัยด้าน data race จาก type system

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python` / `Java` / `C`

---

### Rust vs. Python

| Aspect | Rust | Python |
|-|-|-|
| Syntax | method chaining `.iter().filter(..).map(..).collect()` และ closure แบบ `\|number\| number * 2` | list comprehension `[x*x for x in numbers if ...]` หรือ `map(lambda x: ..., filter(...))` |
| Semantics / Behavior | adapter (`map`, `filter`) เป็น lazy ทำงานเมื่อมี consumer (`collect`, `fold`, `sum`) เรียกเท่านั้น | `map`/`filter` และ generator expression เป็น lazy แต่ list comprehension เป็น eager สร้าง list ทันที |
| Type System | static + strong ใช้ trait `Iterator`, `Fn*`, `FromIterator` ตรวจชนิดครบตอน compile | dynamic typing ตรวจชนิดตอน runtime ใช้ iterator protocol (`__iter__`, `__next__`) แบบ duck typing |
| Memory Management | ownership + borrowing ไม่มี GC ไม่สร้าง collection ชั่วคราวระหว่าง chain และคืนทรัพยากรผ่าน `Drop` | garbage collector (reference counting + cycle GC) list comprehension สร้าง list ใหม่บน heap |
| Safety | borrow checker ป้องกันการแก้ collection ขณะวน และป้องกัน data race ตั้งแต่ compile time | แก้ list ขณะวนได้โดยไม่ error ที่ compile อาจเกิดพฤติกรรมไม่คาดคิด (`dict` ขึ้น `RuntimeError` ตอนรัน) |

#### Rust Example

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // เลือกเลขคู่ -> ยกกำลังสอง
    let result: Vec<i32> = numbers
        .iter()
        .filter(|number| **number % 2 == 0)
        .map(|number| number * number)
        .collect();

    println!("{:?}", result); // [4, 16, 36]
}
```

#### Python Example

```python
numbers = [1, 2, 3, 4, 5, 6]

# list comprehension (eager)
result = [x * x for x in numbers if x % 2 == 0]

# แบบ lazy ด้วย generator expression
lazy = (x * x for x in numbers if x % 2 == 0)

print(result)        # [4, 16, 36]
print(list(lazy))    # [4, 16, 36]
```

#### Analysis

ทั้งสองภาษารองรับการเขียนแบบ functional และมี lazy iterator เหมือนกัน แต่ต่างกันที่ **เวลาที่ตรวจสอบและวิธีจัดการทรัพยากร** Rust ตรวจชนิดและ borrowing ตอน compile จึงจับข้อผิดพลาดได้ก่อนรัน และแปลง iterator chain เป็น native loop ได้เต็มประสิทธิภาพ (zero-cost abstraction) ส่วน Python ออกแบบให้ยืดหยุ่นและเขียนเร็ว ใช้ dynamic typing และ GC จึงมี overhead จาก interpreter แต่เขียนสั้นและ prototype ได้เร็วกว่า เหตุผลด้านการออกแบบคือ Rust เน้น **safety + performance** ส่วน Python เน้น **productivity + readability**

---

### Rust vs. Java

| Aspect | Rust | Java |
|---|---|---|
| Syntax | method chaining บน iterator และ closure `\|number\| number * 2` | Stream API `numbers.stream().filter(..).map(..).collect(..)` และ lambda `x -> x * 2` |
| Semantics / Behavior | adapter เป็น lazy รอ consumer ประมวลผลทีละ element และรองรับ short-circuit เช่น `find`, `any` | intermediate operation (`filter`, `map`) เป็น lazy รอ terminal operation (`collect`, `reduce`) และรองรับ short-circuit เช่น `findFirst`, `anyMatch` |
| Type System | static + strong ใช้ generics แบบ monomorphization และ trait `Fn`, `FnMut`, `FnOnce` สำหรับ closure | static + strong ใช้ generics แบบ type erasure และ functional interface เช่น `Predicate<T>`, `Function<T,R>` สำหรับ lambda |
| Memory Management | ownership + borrowing ไม่มี GC คืนทรัพยากรผ่าน `Drop` | garbage collector (JVM) stream ไม่สร้าง collection กลางทางเช่นกัน แต่ `Stream<Integer>` มี boxing ทำให้มี object เพิ่มบน heap (`IntStream` ช่วยลดได้) |
| Safety | borrow checker ป้องกัน iterator invalidation ตั้งแต่ compile time | แก้ collection ขณะวนจะเกิด `ConcurrentModificationException` ตอน runtime และ lambda จับได้เฉพาะตัวแปร effectively final |

#### Rust Example

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|number| **number % 2 == 0)
        .map(|number| number * number)
        .collect();

    println!("{:?}", result); // [4, 16, 36]
}
```

#### Java Example

```java
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
        List<Integer> numbers = List.of(1, 2, 3, 4, 5, 6);

        List<Integer> result = numbers.stream()
            .filter(x -> x % 2 == 0)
            .map(x -> x * x)
            .collect(Collectors.toList());

        System.out.println(result); // [4, 16, 36]
    }
}
```

#### Analysis

Java Stream API มีแนวคิดใกล้เคียง Rust iterator มาก คือเป็น lazy pipeline ที่แยก adapter ออกจาก consumer ความต่างสำคัญอยู่ที่ **ชั้นล่างของภาษา** Java รันบน JVM พร้อม GC และ generics แบบ type erasure จึงมี overhead จาก boxing และ object allocation ส่วน Rust คอมไพล์เป็น native และตรวจ ownership ตอน compile จึงไม่ต้องมี runtime ช่วยเก็บกวาดหน่วยความจำ และป้องกันบั๊กอย่างการแก้ collection ขณะวนได้ตั้งแต่ก่อนรัน การออกแบบของ Java เน้น **ความเรียบง่ายและ portability** (write once, run anywhere) ขณะที่ Rust เน้น **การควบคุมทรัพยากรโดยไม่เสียความปลอดภัย**

---

### Rust vs. C

| Aspect | Rust | C |
|---|---|---|
| Syntax | method chaining บน iterator และ closure `\|number\| number * 2` | ไม่มี syntax พิเศษ ใช้ `for` loop เอง หรือส่ง function pointer เช่น `int (*f)(int)` |
| Semantics / Behavior | adapter เป็น lazy ทำงานเมื่อมี consumer เรียก | ไม่มี lazy evaluation ในตัว ทุกอย่างเป็น eager ต้องออกแบบ state และ `next()` เองหากต้องการ |
| Type System | static + strong มี generics, trait และ closure ที่คอมไพเลอร์อนุมานชนิดให้ | static แต่ weak ไม่มี generics จริง (มีเพียง macro และ `_Generic` ที่จำกัด) และไม่มี closure ต้องใช้ `void*` ส่งข้อมูลซึ่งเสียความปลอดภัยด้านชนิด |
| Memory Management | ownership + borrowing คืนทรัพยากรอัตโนมัติผ่าน `Drop` | จัดการเอง (`malloc`/`free`) ผลลัพธ์แต่ละขั้นต้องจอง buffer เองและคืนเอง |
| Safety | ใน safe Rust compiler ป้องกัน use-after-free, dangling reference และ data race | ไม่มีการป้องกัน เสี่ยง buffer overflow, use-after-free, dangling pointer และ undefined behavior |

#### Rust Example

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|number| **number % 2 == 0)
        .map(|number| number * number)
        .collect();

    println!("{:?}", result); // [4, 16, 36]
}
```

#### C Example

```c
#include <stdio.h>

int main(void) {
    int numbers[] = {1, 2, 3, 4, 5, 6};
    int n = sizeof(numbers) / sizeof(numbers[0]);

    int result[6];
    int count = 0;

    // เลือกเลขคู่ -> ยกกำลังสอง ด้วย loop เอง
    for (int i = 0; i < n; i++) {
        if (numbers[i] % 2 == 0) {
            result[count++] = numbers[i] * numbers[i];
        }
    }

    for (int i = 0; i < count; i++) {
        printf("%d ", result[i]); // 4 16 36
    }
    printf("\n");
    return 0;
}
```

#### Analysis

C ไม่มีแนวคิด iterator หรือ higher-order programming ในตัว ผู้เขียนต้องคุม loop, ขนาด buffer และหน่วยความจำเองทั้งหมด ซึ่งให้ประสิทธิภาพและการควบคุมสูงสุด แต่เปิดช่องให้เกิดบั๊กร้ายแรง เช่น เขียนเกินขอบ array Rust จึงถูกออกแบบมาเพื่อ **คงประสิทธิภาพระดับ C แต่ย้ายความรับผิดชอบด้านความปลอดภัยไปให้ compiler** ด้วย ownership และ borrow checker พร้อมเพิ่ม abstraction ระดับสูงอย่าง iterator ที่ถูกแปลงเป็นโค้ดใกล้เคียง loop ที่เขียนด้วยมือ จึงได้ทั้งความกระชับและความเร็วโดยไม่ต้องแลกกับความปลอดภัย

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
