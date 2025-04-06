# Nintendo Emulator in Rust

Learn Rust by building an NES emulator — the third generation of a long-running experiment in recreating the same console across different languages, goals, and phases of life.

---

## 🧬 Emulator Generations

1. **2007 in C++98**: The impulse build.  
   It started during finals week. I had a sudden urge to see Mario jump on screen — more than I wanted to study. I stayed up til 4am for multiple nights, skipped at least one final, and hacked together enough CPU, GPU, APU, and controller logic to play some *Super Mario Bros.*  
   **Goal**: *Just make it work.* A scrappy, late-night systems adventure. Pure unadulterated dopamine.

2. **2015 in C++14**: The performance pass.  
   This version focused on speed and compatibility. I experimented with static vs dynamic polymorphism, optimized virtual dispatch and CPU cache layout, and implemented multiple memory mappers.  
   Supporting more games also meant deeper dives into the NES’s quirks — from complex PPU rendering tricks to more advanced APU audio behavior.  
   Getting *Mega Man* and *Zelda* running was especially fun.  
   **Goal**: *Make it fast. Support more games. Explore hardware quirks.* Pure OCD.

3. **2025 in Rust**: The teaching edition.  
   This version is all about learning a new language. It’s a Rust codebase with a focus on readability, commentary, and syntax explanation optimized for developers coming from C or Go.  
   **Goal**: *Make it educational. Learn Rust with AI.*

---

## 🍄 Super Mario Bros: The NES "Hello, World"

The first test case is *Super Mario Bros.*  

It's a simple ROM and it's iconic, touching every sub-system: CPU, PPU, APU, input, memory mapping.  

The goal is to get the **first level** of gameplay running. If Mario walks and jumps, makes that jump and brick hit sounds, the emulator has a working vertical slice, the **walking skeleton**.

Yes, there are official test ROMs for hardware accuracy and testing, *but Mario is more fun.*

---

## 📚 What This Is

- A readable, well-commented emulator that teaches Rust in context  
- Prioritizes clarity over speed or completeness  
- Built with AI tools (Windsurf + LLMs) to explore new ways of learning and pair-programming

---

## 🧭 Who This Is For

- Rust learners from C/Python/Go  
- Emulator hobbyists  
- Code-readers and tinkerers

---

## 🎯 Project Goals

- Build a working core  
- Use Mario as a first test  
- Explain Rust clearly  
- Skip premature optimization  
- Keep it fun
