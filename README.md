# FLAPPY PONG (Project Project / Flames)

---

## 1. Team Identity

### 1.1 Studio / Group Name
`Flames`

### 1.2 Team Members

| Name | Primary Role | Secondary Role | Strengths Brought to the Project |
|------|-------------|---------------|----------------------------------|
| Unnati S Singh | Electronics / fabrication | Documentation | Material Handling, Hardware |
| Suyog Tandel | coding/fabrication | Electronics | Material Handling, software |
| Aadit deshmukh | Raspi setup | documentation | Documentation |
| Rushikesh deore | raspi installnation | documentation | Documentation |

---

### 1.3 Project Title
`"Flappy Pong: A Gravity-Based Interactive Ping Pong Game"`

<img width="1600" height="1131" alt="image" src="https://github.com/user-attachments/assets/c64bfbd4-b3b7-43d9-83ad-c203a5aa11bc" />

### 1.4 One-Line Pitch
`A gravity-driven ping pong game inspired by Flappy Bird, controlled using touch sensors on Raspberry Pi with real-time interactive gameplay.`

### 1.5 Expanded Project Idea

Flappy Pong is a two-player game that merges Pong with Flappy Bird. Both paddles are subject to constant gravity and fall toward the bottom of the screen unless the player taps their physical capacitive touch sensor to apply an upward impulse — just like flapping. The ball speeds up with every successful rally hit, capped at a maximum velocity, creating escalating pressure as a round goes on.  

The game runs as a native Rust application using the Raylib graphics library at 60 FPS on a Raspberry Pi. The `rppal` crate reads the GPIO-connected sensors every frame with rising-edge detection so a single tap fires exactly one flap. Audio is generated entirely in software, sine waves and frequency sweeps are synthesized at 44,100 Hz into WAV buffers at startup, with distinct sounds for paddle hits, wall bounces, scoring, and menu navigation. A difficulty selector (Easy / Medium / Hard) adjusts gravity, impulse strength, ball speed, and paddle height before each match.

---

## 2. Inspiration

### 2.1 References

| Source Type | Title / Link | What Inspired You |
|------------|-------------|------------------|
| Game | Flappy Bird (Dong Nguyen, 2013) | Gravity + tap mechanic — constant falling with discrete upward impulses |
| Game | Pong (Atari, 1972) | Two-player ball rally with left/right paddles and scoring |
| Hardware | Raspberry Pi GPIO projects | Using physical sensors as game controllers instead of buttons |
| Concept | Increasing difficulty via speed | Ball accelerates with each rally — pressure builds naturally without changing the rules |

### 2.2 Original Twist

Classic Pong gives paddles free vertical movement. Here, paddles are in permanent free-fall — the player must actively fight gravity on every frame or the paddle crashes to the floor. This forces players to split their attention between timing their flaps and tracking the ball, turning a passive "move paddle to ball" game into an active "survive gravity while playing Pong" challenge. No prior game combines these two mechanics with physical hardware sensors.

---

## 3. Project Intent

### 3.1 User Journey (Story Format)

Two players face the monitor. On the left, a small blue lollipop-shaped paddle; on the right, a green one. Both are already falling toward the bottom of the screen.  

Player 1 reaches out and taps the capacitive sensor on their side. The blue paddle flaps upward. Player 2 does the same — the green paddle catches itself mid-fall. The ball launches from center screen.  

It hurtles toward Player 1. They tap just in time — the paddle swings and deflects the ball back. The game announces "RALLY x1." The ball returns faster. They keep tapping, keep fighting gravity, keep hitting. By rally 6 the ball is a yellow streak and both players are laughing and panicking at once.  

The ball flies past Player 2's paddle. A score chime plays and "1" appears in the corner for Player 1. The ball resets. They go again.

---

## 4. Definition of Success

### 4.1 Definition of "Usable"

The game is usable when two players can each pick up a sensor, understand "tap to flap" within one rally, and play a complete match (scoring points, switching serves, reading scores on screen) without any explanation beyond watching a 10-second demo.

### 4.2 Minimum Usable Version

Both sensors trigger paddle flaps over GPIO. The ball bounces, speeds up on each hit, and awards a point when it passes either edge. Scores display. The game resets after a point. That loop — flap, rally, score, reset — is the complete core experience.

### 4.3 Stretch Features

Online leaderboard display  
Configurable winning score (first to 5, 10, etc.)  
Power-ups that temporarily change gravity or ball size  
Multiplayer over network (two Raspberry Pi units)  
Animated menu with live demo ball  
Custom paddle skin selector  

---

## 5. System Overview

### 5.1 Project Type

[x] Electronics-based  
[x] Sensor-based  
[x] Screen/UI-based  
[x] Game logic based  
[x] Sound-based  

---

### 5.2 High-Level System Description

**Input:** Two capacitive touch sensors are wired to GPIO 17 (Player 1) and GPIO 27 (Player 2) on the Raspberry Pi. Tapping a sensor sends its output pin HIGH. A keyboard fallback (TAB for P1, LEFT SHIFT for P2) allows play without hardware.  

**Processing:** A Rust application (`main.rs`) runs a 60 FPS game loop using Raylib. Each frame it reads GPIO pin states via `rppal`, applies rising-edge detection, updates paddle physics (gravity accumulation, impulse application, boundary clamping), moves the ball, checks wall and paddle collisions, calculates speed-boosted deflections, and evaluates scoring conditions. Audio synthesis (sine and sweep wave generation) runs once at startup; sound buffers are played on game events.  

**Output:** The game renders at 1280 × 720 px fullscreen over HDMI to a monitor. Rendered elements include: two animated bat-shaped paddles with swing animation, a ball with a fading yellow trail, a dashed center line, live scores, and a rally counter. Sound effects play through the 3.5 mm audio jack.  

**Physical structure:** The Raspberry Pi sits connected to a breadboard. Two capacitive touch sensor modules are positioned within reach of each player, wired to the GPIO header via jumper cables. The monitor sits facing both players.  

**App interaction:** No external app. Everything runs locally on the Raspberry Pi — the game is self-contained.

---

### 5.3 Input / Output Map

| System Part | Type | What It Does |
|------------|------|--------------|
| Capacitive touch sensor — P1 (GPIO 17) | Digital input | Detects Player 1 tap; applies upward impulse to P1 paddle |
| Capacitive touch sensor — P2 (GPIO 27) | Digital input | Detects Player 2 tap; applies upward impulse to P2 paddle |
| Keyboard TAB / LEFT SHIFT | Digital input (fallback) | Same impulse trigger without hardware |
| Arrow keys / A-D | Digital input | Navigate difficulty selection in menu |
| ENTER / SPACE | Digital input | Confirm difficulty and start game |
| ESC | Digital input | Return to menu from gameplay |
| Raspberry Pi — main.rs (Rust) | Processing | 60 FPS game loop: physics, collision, scoring, audio |
| HDMI monitor (1280 × 720) | Visual output | Fullscreen game rendering |
| 3.5 mm audio jack | Audio output | Synthesized hit, wall, score, and menu sound effects |
| On-screen scoreboard | Visual output | Live P1 / P2 integer scores |
| On-screen rally counter | Visual output | Shows "RALLY x N" when consecutive hits ≥ 4 |

---

## 6. System Design, Sketches and Visual Planning

### 6.1 Concept Architecture / Schematic
![Architecture](assests/architecture.png)

### 6.2 Labeled Build Sketch / Architecture / Flow Diagram / Algorithm
mainloop game
<br>
![Flowchart](assests/flowchart.mainloop.png)
<br>
game system intraction 
<br>
![Flowchart](assests/flowchart.system.png)


### 6.3 Approximate Dimensions

| Dimension | Value |
|----------|------|
| Length | 16 cm (breadboard + sensor spread) |
| Width | 16 cm |
| Height | 8 cm (Raspberry Pi + wiring clearance) |
| Estimated weight | ~400 g |

---

## 7. Electronics Planning

### 7.1 Electronics Used

| Component | Quantity | Purpose |
|----------|----------|--------|
| Raspberry Pi (any 40-pin model) | 1 | Main game controller and compute |
| Capacitive touch sensor module | 2 | Player 1 and Player 2 physical input |
| Breadboard (full size) | 1 | Wiring hub for sensors and GPIO |
| Jumper wires (M-F) | ~10 | Connect sensors to GPIO header |
| HDMI monitor | 1 | Display game at 1280 × 720 |
| USB-C power supply (5 V / 3 A) | 1 | Power Raspberry Pi |
| 3.5 mm speaker or headphones | 1 | Audio output for game sounds |

### 7.2 Wiring Plan

The Raspberry Pi's 3.3 V pin (Pin 1) supplies both capacitive touch sensor modules via the breadboard's power rail. Each sensor's signal output (SIG) connects to a dedicated GPIO input pin: Sensor P1 SIG to GPIO 17, Sensor P2 SIG to GPIO 27. Both sensor GND pins connect to a common GND pin on the Raspberry Pi (Pin 6 or Pin 14) through the breadboard ground rail.  

The Raspberry Pi reads each pin every frame using `rppal::gpio::InputPin::is_high()`. Since the sensors output a clean digital HIGH when touched, no pull-up or pull-down resistors are required. The monitor connects via HDMI and the speaker via the 3.5 mm audio jack. The board is powered by a 5 V USB-C supply — no battery pack needed since the game has no motors.

### 7.3 Circuit Diagram
7.3 Circuit Diagram

===============================
Raspberry Pi 4 GPIO Connections
===============================

Power Connections:
------------------
Pin 1  (3.3V)  -------->  VCC (Sensor P1)
Pin 1  (3.3V)  -------->  VCC (Sensor P2)

Pin 6  (GND)   -------->  GND (Sensor P1)
Pin 6  (GND)   -------->  GND (Sensor P2)

NOTE:
Both sensors share the same ground (COMMON GROUND).

----------------------------------------------

Input Signal Connections:
-------------------------
GPIO 17 (Pin 11)  ------>  SIG (Sensor P1)
GPIO 27 (Pin 13)  ------>  SIG (Sensor P2)

----------------------------------------------

System Behavior:
----------------
• When Sensor P1 is touched:
  GPIO 17 reads HIGH → Player 1 paddle moves upward

• When Sensor P2 is touched:
  GPIO 27 reads HIGH → Player 2 paddle moves upward

• When not touched:
  GPIO pins remain LOW

----------------------------------------------


### 7.4 Power Plan

| Question | Response |
|---------|---------|
| Power source | 5 V USB-C wall adapter |
| Voltage required | 5 V for Raspberry Pi; 3.3 V (from Pi's own pin) for both sensors |
| Current concerns | Minimal — no motors or high-draw components. Sensor draw is <5 mA each. Total system ~1.5 A max. |
| Safety concerns | No Li-ion batteries. No high voltages. Standard GPIO precautions: confirm sensor is 3.3 V rated before wiring; avoid connecting 5 V to GPIO input pins. |

---

## 8. Software Planning

### 8.1 Software Tools

| Tool / Platform | Purpose |
|----------------|--------|
| Rust (stable toolchain) | Primary game language |
| Raylib (`raylib` crate) | 2D graphics, windowing, audio device, input |
| rppal (`rppal` crate) | Raspberry Pi GPIO access from Rust |
| Nix / flake.nix | Reproducible development environment |
| Cargo | Rust build system and dependency management |

### 8.2 Software Logic / Algorithm

Startup behavior:  
Raylib initializes a 1280 × 720 fullscreen window targeting 60 FPS. The audio device starts and four sound buffers are synthesized in memory: a 480 Hz sine (paddle hit), a 220 Hz sine (wall bounce), a 280→900 Hz sweep (score), and a 360 Hz sine (menu click). GPIO is initialized via `rppal`; if unavailable (non-Pi hardware), the game silently falls back to keyboard-only mode. Initial state is set to `Menu`.  

Input handling:  
Every frame, current GPIO pin states are read for P1 (GPIO 17) and P2 (GPIO 27). Rising-edge detection (`curr == HIGH && prev == LOW`) ensures exactly one impulse fires per physical tap. Keyboard equivalents are checked in parallel (TAB for P1, LEFT SHIFT for P2, arrow keys for menu navigation, ENTER/SPACE to start, ESC to return to menu).  

Sensor reading:  
`rppal::gpio::InputPin::is_high()` is polled each frame. Previous frame state is stored in `p1_prev` / `p2_prev` booleans for edge detection.  

Decision logic:  
In Menu state, arrow keys cycle through Easy / Medium / Hard. ENTER loads `DifficultySettings` (gravity, impulse, ball_speed, paddle_height) and transitions to Playing. In Playing state, each paddle accumulates gravity every frame, clamped to screen bounds with velocity zeroed on boundary contact. On a valid tap edge, `vy = -impulse` is applied. Paddle-ball collision is tested using `check_collision_circle_rec`; the collision only registers if the ball is moving toward that paddle. On hit, speed increases by `SPEED_BOOST_PER_HIT × hit_count`, capped at `MAX_BALL_SPEED` (22). If the ball passes the left edge, P2 scores; if it passes the right edge, P1 scores.  

Output behavior:  
`draw_bat()` renders each paddle as a rotated lollipop bat with swing animation. `draw_trail()` renders a fading yellow trail. Ball color changes based on speed. Scores render in large gray text; rally counter shows in orange when hits ≥ 4.  

Communication logic:  
No network communication. All communication is local GPIO reads within the same board.  

Reset behavior:  
`reset_ball()` re-centers the ball. `reset_paddles()` restores paddle state. ESC returns to Menu.

---

## 9. Bill of Materials

### 9.1 Full BOM

| Item | Quantity | In Kit? | Need to Buy? | Estimated Cost (₹) | Spec | Why This Choice? |
|------|----------|---------|--------------|---------------------|------|------------------|
| Raspberry Pi (40-pin) | 1 | Yes | No | 0 | Any Pi with 40-pin GPIO | Runs Rust natively; GPIO available |
| Capacitive touch sensor | 2 | Yes | No | 0 | 3.3 V digital output | Simple HIGH/LOW output, no ADC needed |
| Breadboard (full size) | 1 | Yes | No | 0 | 830 tie points | Wiring hub, no soldering needed |
| Jumper wires (M-F, 20 cm) | 10 | Yes | No | 0 | Standard dupont | Flexible routing to GPIO header |
| HDMI monitor | 1 | Yes | No | 0 | 1280 × 720 or higher | Game renders at this native res |
| 5 V / 3 A USB-C adapter | 1 | Yes | No | 0 | Standard Pi power supply | Stable power, no motors to worry about |
| 3.5 mm speaker or headphones | 1 | Yes | No | 0 | Any | Audio output for synthesized sounds |

---

### 9.2 Material Justification

Capacitive touch sensors were chosen over mechanical push buttons because they require zero actuation force, have no moving parts to wear out, and produce a clean digital HIGH signal that maps directly to a GPIO read without debounce circuitry. The entire project requires no purchased components — all parts were available in the lab kit. Rust was chosen over Python for performance: the game loop, physics, and audio synthesis all run at 60 FPS with deterministic timing, which MicroPython or even CPython would struggle to guarantee on a Pi.

---

### 9.3 Items Procured

| Item | Why Needed | Source | Status |
|------|-----------|--------|--------|
| Capacitive touch sensors × 2 | P1 and P2 game input | Lab kit | Received |
| Raspberry Pi | Game compute + GPIO | Lab kit | Received |
| HDMI monitor | Game display | Lab / campus | Received |

---

### 9.4 Budget Summary

| Budget Item | Estimated Cost (₹) |
|------------|-------------------|
| Electronics | 0 (all from kit) |
| Mechanical parts | 0 |
| Fabrication materials | 0 |
| Purchased extras | 0 |
| Contingency | 100 |
| Total | ~100 |

---

### 9.5 Budget Reflection

Cost is essentially zero because the game is software-only with minimal hardware — just two sensor modules and jumper wires. If sensors were unavailable, any two push buttons wired to GPIO would substitute at under ₹20 total. The only potential cost is a dedicated small speaker for better audio than a laptop headphone, estimated at ₹150–200.

---

## 10. Planning the Work

### 10.1 Team Working Agreement

Tasks are divided by domain: Mrugendra owns the Rust codebase, game logic, and audio; Jyoti owns the hardware wiring, sensor testing, and physical setup. Decisions are made by a quick verbal agreement — if there's a disagreement, the person whose domain it falls under gets final say. Progress is checked at the end of each 2-hour block against the milestone checklist. If a task is delayed, we immediately flag it and decide whether to simplify the feature or swap tasks. Documentation is updated live by Mrugendra at the end of each session.

---

### 10.2 Task Breakdown

| Task ID | Task | Owner | Est. Hours | Deadline | Dependency | Status |
|--------|------|------|------------|----------|------------|--------|
| T1 | Finalize game concept and mechanics | all | 1 | Hour 1 | None | Done |
| T2 | Set up Rust + Raylib project structure | suyog | 1 | Hour 2 | T1 | Done |
| T3 | Wire sensors to GPIO, test signal | unnati | 1 | Hour 2 | T1 | Done |
| T4 | Implement paddle physics (gravity + impulse) | rushikesh,suyog | 2 | Hour 4 | T2 | Done |
| T5 | Integrate GPIO input via rppal | unnati,suyog | 1 | Hour 4 | T3, T4 | Done |
| T6 | Ball physics, wall bounce, collision | suyog,aadit | 2 | Hour 6 | T4 | Done |
| T7 | Scoring logic and ball reset | unnati,rushikesh | 1 | Hour 6 | T6 | Done |
| T8 | Audio synthesis and sound events | all | 1 | Hour 7 | T6 | Done |
| T9 | Menu + difficulty selector | all | 1 | Hour 8 | T7 | Done |
| T10 | Paddle art (bat renderer) and trail | suyog,aadit | 1 | Hour 9 | T4 | Done |
| T11 | Playtesting and bug fixing | aadit,rushikesh | 2 | Hour 11 | T9, T10 | Done |
| T12 | Documentation and README | unnati | 2 | Hour 12 | All | Done |

---

### 10.3 Responsibility Split

| Area | Main Owner | Support Owner |
|------|------------|---------------|
| Concept | Suyog | Unnati |
| Electronics / wiring | Unnati | Rushikesh |
| Coding (Rust / game logic) | Suyog | Aadit |
| Mechanical build | all | all |
| Testing / playtesting | All | All |
| Documentation | Unnati | suyog |

---

## 11. Hour Milestones

### 11.1 8-Hour Plan

**Bi-Hour 1 — Plan and De-risk**

[x] Idea finalized (Flappy Pong — gravity paddles + physical sensors)  
[x] Core interaction decided (capacitive tap = flap impulse)  
[x] Sketches made  
[x] BOM completed  
[x] Purchase needs identified (none — all in kit)  
[x] Key uncertainty identified (GPIO timing and edge detection in Rust)  
[x] Basic feasibility tested (Raylib hello-world on Pi)  

---

**Bi-Hour 2 — Build Subsystems**

[x] Electronics tests completed (sensors read correctly on GPIO 17 and 27)  
[x] Paddle physics implemented and visible  
[x] Keyboard-based flap working before GPIO integration  
[x] Ball movement and wall bounce working  

---

**Bi-Hour 3 — Integrate**

[x] GPIO edge detection integrated with paddle impulse  
[x] Both sensors driving both paddles live  
[x] Collision detection and scoring working  
[x] Audio buffers synthesized and playing on events  

---

**Bi-Hour 4 — Refine and Finish**

[x] Bat renderer (lollipop paddle art) complete  
[x] Ball trail and speed-color interpolation added  
[x] Difficulty menu with stats display complete  
[x] Playtesting completed  
[x] Documentation completed  
[x] Final build ready and running fullscreen  

---

### 12.2 Update Log

| Day | Planned Goal | What Actually Happened | What Changed | Next Steps |
|-----|-------------|----------------------|-------------|-----------|
| Day 1 | Set up project, wire sensors, implement physics | Project scaffolding done; sensors wired and reading cleanly on GPIO; paddle gravity + impulse working on keyboard | Decided to add rising-edge detection immediately to avoid double-flap issues | Move to ball physics and collision |
| Day 1 | Ball physics, collision, scoring, GPIO integration | All core mechanics done; GPIO integrated with clean edge detection | Added `SPEED_BOOST_PER_HIT` constant after playtesting felt too easy | Audio synthesis and menu |
| Day 1 | Audio, menu, difficulty settings | Audio synthesis complete with distinct tones per event; menu with 3 difficulty tiers working | Switched from flat ball color to speed-interpolated color for better player feedback | Paddle art and trail |
| Day 1&2 | Paddle art, trail, playtesting, documentation | Bat renderer complete with swing animation; ball trail working; multiple playtest sessions done | Fixed a paddle swing decay bug (was using 0.9, changed to 0.86 for snappier feel) | Final documentation |

---

## 13. Risks and Unknowns

### 13.1 Risk Register

| Risk | Type | Likelihood | Impact | Mitigation Plan | Owner |
|------|------|------------|--------|-----------------|-------|
| GPIO read timing causes double-flap on single tap | Technical | Medium | High | Rising-edge detection (`curr && !prev`) implemented on every frame | suyog |
| `rppal` fails to initialize on non-Pi hardware | Technical | High | Low | Silent fallback to keyboard-only mode already coded | suyog |
| Sensor stays HIGH for multiple frames (noise) | Hardware | Medium | Medium | Edge detection handles this — only the first HIGH frame fires the impulse | unnati |
| Raylib audio device fails to init on Pi | Technical | Low | Medium | `expect()` panics with a clear message; test audio init as first step | aadit |
| Ball speed cap (`MAX_BALL_SPEED`) feels too slow or too fast | Gameplay | Medium | Medium | Constant is easily adjustable; playtested and tuned to 22.0 | all |
| Monitor resolution mismatch with 1280×720 target | Hardware | Low | Low | Raylib toggles fullscreen; native resolution scales gracefully | rushikesh |

---

### 13.2 Biggest Unknown Right Now

Whether the capacitive sensors respond consistently for different players' hand sizes and tap styles. Some players tap lightly (sensor may not trigger), others hold (sensor stays HIGH, but edge detection handles this). Extended playtesting with different people will reveal if sensor sensitivity needs hardware adjustment (a small trim potentiometer on the sensor module controls this).

---

## 14. Testing

### 14.1 Technical Testing Plan

| What Needs Testing | How You Will Test It | Success Condition |
|-------------------|---------------------|------------------|
| GPIO sensor input | Println each frame when pin reads HIGH; tap sensor 10 times | Exactly 10 prints, no doubles, no missed taps |
| Paddle impulse | Tap sensor rapidly; observe paddle on screen | Paddle flaps once per tap; no multi-flap artifacts |
| Ball-paddle collision | Position paddle directly in ball path at slow speed | Ball deflects correctly; speed increases; swing animation plays |
| Scoring | Let ball pass each edge deliberately | Score increments for correct player; ball resets to center |
| Difficulty change | Play one rally on each difficulty | Noticeable difference in gravity feel and ball speed |
| Audio | Run game without monitor; listen only | Four distinct sounds play on correct events |
| Fullscreen on Pi | Run `cargo run` on Pi connected to monitor | Game launches fullscreen, no windowed fallback |

---

### 14.2 Testing and Debugging Log

| Date | Problem Found | Type | What You Tried | Result | Next Action |
|------|--------------|------|----------------|--------|------------|
| Day 1 | Double-flap firing on single tap | Software | Added `p1_prev` / `p2_prev` rising-edge detection | Fixed completely | Monitor for regression |
| Day 1 | Ball passing through paddle at high speed | Software | Tightened collision rect check; clamped ball position after collision | Fixed | Test at MAX_BALL_SPEED |
| Day 1 | Paddle falls through floor at high gravity (Hard mode) | Software | Added `p.y.clamp(0.0, SCREEN_H - p.height)` | Fixed | Confirm velocity also zeroes at boundary |
| Day 1 | Audio device init crash on test laptop (no audio) | Software | Wrapped in `.expect()` with clear message; tested audio availability first | Acceptable — Pi always has audio device | Document as known non-Pi behavior |
| Day 1 | Swing angle not fully decaying between rallies | Software | Changed decay factor from 0.9 to 0.86; added `< 0.01` zero-clamp | Fixed — paddle returns to neutral cleanly | — |

---

## 15. Build Documentation

### 15.1 Fabrication Process

No structural fabrication was required. The project is entirely electronics and software on an existing Raspberry Pi board with off-the-shelf sensor modules.  

Wiring: Sensor modules were connected to the Raspberry Pi GPIO header using female-to-female jumper wires. Sensor P1 SIG wire to GPIO 17 (Pin 11), Sensor P2 SIG to GPIO 27 (Pin 13), both VCC wires to the 3.3 V pin (Pin 1) via the breadboard power rail, both GND wires to GND (Pin 6) via the breadboard ground rail.  

Software build environment: The Nix flake (`flake.nix` + `shell.nix`) defines a reproducible shell with the correct Rust toolchain and system dependencies for Raylib on ARM. Running `nix develop` (or `direnv allow` with `.envrc`) drops into the build shell. `cargo build --release` compiles the game; `cargo run --release` launches it.  

Physical layout: The Raspberry Pi and breadboard were placed on a flat surface between the two players. Each sensor was positioned approximately 30 cm from the board, at comfortable arm reach for each player. The monitor was placed facing both players centrally. No enclosure was built — the wiring is left visible as part of the hackathon prototype aesthetic.  

Revisions: Initial sensor placement had both sensors too close together, causing accidental cross-triggering when players leaned in. Sensors were moved 40 cm apart as a simple fix.

---

## 16. Build Photos

Add photos here as the build progresses. Suggested shots:  
Breadboard wiring close-up (sensors connected to GPIO)  
Both sensors in playing position on table  
Game running on monitor — menu screen  
Game running — active rally with ball trail visible  
Both players in action shot  

```md
### Game UI (Running on Monitor)
![Game UI](assests/ui.png)

### Circuit Connections (Breadboard + Raspberry Pi)
![Wiring](assests/wiring.png)

### Raspberry Pi GPIO Pin Configuration
![GPIO](assests/gpio.png)
```

---

## 17. Final Outcome

### 17.1 Final Description

Flappy Pong is a complete, playable two-player arcade game running natively on a Raspberry Pi at 60 FPS. Two players use capacitive touch sensors wired to GPIO to control bat-shaped paddles that fall under gravity and must be actively flapped upward to intercept a rallying ball that accelerates with every hit. The game includes a difficulty menu, procedurally synthesized audio, animated paddle art with swing feedback, a speed-reactive ball trail, a rally counter, and clean score tracking with ball reset.

---

### 17.2 What Works Well

The rising-edge GPIO input is rock-solid — no double-flaps, no missed taps across hundreds of rallies  
The speed ramp (`SPEED_BOOST_PER_HIT = 0.18`) creates natural tension without changing any rules — rallies feel increasingly frantic on their own  
Audio is generated purely in code with no asset files, keeping the project fully self-contained  
The bat renderer (`draw_bat`) produces a distinctive, visually satisfying paddle that clearly communicates hits through its swing animation  
The Nix flake ensures the build environment is reproducible on any Pi without manual dependency management  

---

### 17.3 What Still Needs Improvement

No winning condition — the game scores indefinitely; a "first to N" win state would complete the loop  
Sensor sensitivity varies with different players' tap styles; a calibration step or sensitivity trim would help  
No visual indication of which sensor belongs to which player at the start — brief on-screen "tap your sensor" prompt would fix this  
The menu has no animated preview of gameplay to explain the mechanic to first-time players  

---

### 17.4 What Changed From the Original Plan

The original concept (from the README template) described an RC car with a projector and camera tracking system. During the hackathon, the team pivoted entirely to a self-contained software game on the Raspberry Pi using the available capacitive sensors as physical controllers. The motor driver, BO motors, buck converter, and battery pack from the original BOM were all dropped. The core innovation shifted from mixed-reality projection to the Flappy Bird × Pong physics mechanic and the physical sensor-as-controller interaction. The Rust + Raylib stack was chosen specifically for its performance on Pi hardware.

---

## 18. Reflection

### 18.1 Team Reflection

The team moved fast by keeping the scope tight — one mechanic (flap-pong), one platform (Pi + sensors), one language (Rust). The biggest time sink was the Raylib + rppal integration in Nix, which required careful system dependency pinning. Once that was solved, the actual game logic came together quickly. Mrugendra drove the code at high speed; Jyoti kept the hardware side reliable and caught the sensor-spacing bug early through hands-on testing. Documentation fell slightly behind during the build crunch but was completed in the final session. If task ownership had been more strictly parallel from hour one, we could have recovered that time.

---

### 18.2 Technical Reflection

Electronics: Capacitive sensors output clean digital signals that need no additional circuitry — a lesson in choosing the right sensor for the job. GPIO on the Pi is reliable but must be polled correctly; edge detection is essential for discrete event inputs.  

Coding: Rust's ownership model initially slowed development, but the borrow checker caught bugs at compile time that would have been runtime issues in Python. Audio synthesis from scratch was straightforward and educational.  

Integration: The biggest integration challenge was proper initialization order of Raylib audio system.  

Fabrication: No fabrication required — allowed more focus on software quality.  

---

### 18.3 Design Reflection

Designing for delight: The swing animation had the biggest impact on user satisfaction  
Clarity: Difficulty stats improved user understanding  
Physical interaction: Touch sensors enhanced immersion  
Iteration: Rally counter tuning improved engagement  

---

### 18.4 If We Had One More Hour

Add a win condition (first to 7 points) and a calibration system for sensors.
