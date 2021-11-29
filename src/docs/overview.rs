//!
//! There are two main objectives with creating Void Money:
//!
//! - Build an enjoyable game which simulates economics and business.
//! - Build a game engine to create enjoyable, multi-platform, high-agency games.
//!
//! # Engine
//!
//! The engine is designed to build games with the following criteria:
//!
//!  - Agency & Dominion
//!  - Thrill
//!  - High personalization
//!  - Performance & Reliability
//!  - Accessibility
//!
//! Each one of these is defined as such:
//!
//! # **Agency & Dominion**
//! We want to give the player freedom. This is found within two categories:
//!
//!  - Agency - The freedom for players to choose their own path.
//!  - Dominion - The control or impact the player has over the game world.
//!
//! # **Thrill**
//! The game should be enjoyable to players. This involves two aspects:
//!
//!  - Journey - Does the player enjoy their time while going for something?
//!  - Fulfillment - Does the player feel satisfied or accomplished?
//!
//! This has been exploited at times in the industry. The aim is not to create an addictive product,
//! but to create a product which is enjoyed and satisfying to the player.
//!
//! A business simulator for example, while the project does not provide real-life specific advise,
//! may hopefully cultivate a greater understanding of economics. The goal is that the project leave
//! the player satisfied that they played it.
//!
//! We can look at this three ways:
//!
//! - Past - Does the player want to play the product? Does it look exciting or interesting?
//! - Present - Does the player enjoy the game while playing?
//! - Future - Is the player happy they played the game? Do they feel it was a good break? Has it
//! cultivated a deeper interest or knowledge in beneficial topics, like economics?
//!
//! Often, other games may only focus on one or two of these. The goal for this project is to focus
//! on all three.
//!
//! # **High Personalization**
//!
//! The engine is designed with the mindset that **every player is important**.
//!
//! Every player will chart a unique path. Every player will create slightly, or drastically
//! different things. Every player may use this product slightly, or drastically different.
//!
//! The engine is to be designed to let the player be an individual. And for a variety of use cases.
//! The player though is responsible for how they use it.
//!
//! # **Performance & Reliability**
//!
//! In the game industry today, many video games come out buggy or with lower quality.
//!
//! The goal is to ship a product with high quality. Performance and reliability are difficult to
//! notice when they work, but their absence is awful. The engine is the bedrock for performance and
//! reliability.
//!
//! Rust was selected as the language of choice for the engine. It's a very high performance
//! language, but also it enforces reliability. It is strict in ensuring code is not just written
//! to get something working quickly. It makes writing unit tests much easier. It makes
//! documentation much easier. It can provide memory safety when utilized appropriately.
//!
//! # **Accessibility**
//!
//! Accessibility means many different players can play the game. This comes in three categories:
//!
//!  - Usability
//!  - Safety
//!  - Multi-Device Support
//!
//! **Usability**
//! Usability involves **people being able to use the product**. This single word can bring
//! individuals of varying cultures, age groups, demographics, or who may or may not have a
//! disability of some kind, to be able to use and enjoy the product. The goal is for as many
//! individuals to be able to use the product as possible.
//!
//! **Safety**
//! Safety involves **people not being harmed by the product**. This goes along with usability.
//! While ensuring legal protection in providing the product, it should go beyond that. The product
//! should actively seek to promote the safety of its users, even beyond where required by law.
//! Users do have their freedoms to use the product as they wish, and are responsible for such, but
//! measures can certainly be taken to help, even beyond where required legally. This also crosses
//! over with performance & reliability.
//!
//! **Multi-Device Support**
//! Multi-Device Support involves allowing users of different devices to use the product. This is
//! two-fold:
//!
//! - Devices of different platforms
//! - Devices of different input methods
//! - Devices of different output methods
//! - Devices of different capabilities & permissions
//!
//! Many game engines today target very high performance machines. They require large investments
//! from their players, or are only exclusively available on this or that platform.
//!
//! *Platforms* are different operating systems or runtime environments. The engine's WebGL platform
//! has been selected first to maximize the number of devices which can run or use it, as the web is
//! very irrespective of platform.
//!
//! *Input Methods* involve how the user performs actions in the game. A focus on keyboard, mouse
//! and touch is emphasized, and assistive devices will be looked into.
//!
//! *Output Methods* involve things like visuals, audio, tactile response, etc. Besides providing a
//! richer experience, providing multiple forms of output stimuli is beneficial to communicating
//! the game world with players. This may be beneficial to assist with disability, or to provide
//! convenience for players who may have less access to one or the other (audio on mute, or a
//! device which may not have as powerful graphics capabilities, etc.)
//!
//! *Capabilities & Permissions* involve what the user's device can do, or what the user has allowed
//! to do. The user should be able to play the game on a wide variety of devices. The experience
//! should be enjoyable on many devices. Similarly, if the user has opted out of some permissions,
//! the game should provide alternatives, and/or operate as much as it can.
