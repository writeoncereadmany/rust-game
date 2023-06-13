#![allow(dead_code, unused_variables, non_upper_case_globals)]

// offsets from C
pub const C_mod: f32 = 1.0;
pub const Db_mod: f32 = 16.0 / 15.0;
pub const D_mod: f32 = 9.0 / 8.0;
pub const Eb_mod: f32 = 6.0 / 5.0;
pub const E_mod: f32 = 5.0 / 4.0;
pub const F_mod: f32 = 4.0 / 3.0;
pub const Fs_mod: f32 = 45.0 / 32.0;
pub const G_mod: f32 = 3.0 / 2.0;
pub const Ab_mod: f32 = 8.0 / 5.0;
pub const A_mod: f32 = 5.0 / 3.0;
pub const Bb_mod: f32 = 9.0 / 5.0;
pub const B_mod: f32 = 15.0 / 8.0;

// octave scaling factors
pub const Octave_0: f32 = 1.0 / 16.0;
pub const Octave_1: f32 = 1.0 / 8.0;
pub const Octave_2: f32 = 1.0 / 4.0;
pub const Octave_3: f32 = 1.0 / 2.0;
pub const Octave_4: f32 = 1.0;
pub const Octave_5: f32 = 2.0;
pub const Octave_6: f32 = 4.0;
pub const Octave_7: f32 = 8.0;
pub const Octave_8: f32 = 16.0;
pub const Octave_9: f32 = 32.0;

// reference tone is A=440Hz, and middle c derived from that as our starting point
pub const REF_FREQ: f32 = 440.0 / A_mod;

// and now.... all the notes
pub const C0: f32 = REF_FREQ * C_mod * Octave_0;
pub const Db0: f32 = REF_FREQ * Db_mod * Octave_0;
pub const D0: f32 = REF_FREQ * D_mod * Octave_0;
pub const Eb0: f32 = REF_FREQ * Eb_mod * Octave_0;
pub const E0: f32 = REF_FREQ * E_mod * Octave_0;
pub const F0: f32 = REF_FREQ * F_mod * Octave_0;
pub const Fs0: f32 = REF_FREQ * Fs_mod * Octave_0;
pub const G0: f32 = REF_FREQ * G_mod * Octave_0;
pub const Ab0: f32 = REF_FREQ * Ab_mod * Octave_0;
pub const A0: f32 = REF_FREQ * A_mod * Octave_0;
pub const Bb0: f32 = REF_FREQ * Bb_mod * Octave_0;
pub const B0: f32 = REF_FREQ * B_mod * Octave_0;

pub const C1: f32 = REF_FREQ * C_mod * Octave_1;
pub const Db1: f32 = REF_FREQ * Db_mod * Octave_1;
pub const D1: f32 = REF_FREQ * D_mod * Octave_1;
pub const Eb1: f32 = REF_FREQ * Eb_mod * Octave_1;
pub const E1: f32 = REF_FREQ * E_mod * Octave_1;
pub const F1: f32 = REF_FREQ * F_mod * Octave_1;
pub const Fs1: f32 = REF_FREQ * Fs_mod * Octave_1;
pub const G1: f32 = REF_FREQ * G_mod * Octave_1;
pub const Ab1: f32 = REF_FREQ * Ab_mod * Octave_1;
pub const A1: f32 = REF_FREQ * A_mod * Octave_1;
pub const Bb1: f32 = REF_FREQ * Bb_mod * Octave_1;
pub const B1: f32 = REF_FREQ * B_mod * Octave_1;

pub const C2: f32 = REF_FREQ * C_mod * Octave_2;
pub const Db2: f32 = REF_FREQ * Db_mod * Octave_2;
pub const D2: f32 = REF_FREQ * D_mod * Octave_2;
pub const Eb2: f32 = REF_FREQ * Eb_mod * Octave_2;
pub const E2: f32 = REF_FREQ * E_mod * Octave_2;
pub const F2: f32 = REF_FREQ * F_mod * Octave_2;
pub const Fs2: f32 = REF_FREQ * Fs_mod * Octave_2;
pub const G2: f32 = REF_FREQ * G_mod * Octave_2;
pub const Ab2: f32 = REF_FREQ * Ab_mod * Octave_2;
pub const A2: f32 = REF_FREQ * A_mod * Octave_2;
pub const Bb2: f32 = REF_FREQ * Bb_mod * Octave_2;
pub const B2: f32 = REF_FREQ * B_mod * Octave_2;

pub const C3: f32 = REF_FREQ * C_mod * Octave_3;
pub const Db3: f32 = REF_FREQ * Db_mod * Octave_3;
pub const D3: f32 = REF_FREQ * D_mod * Octave_3;
pub const Eb3: f32 = REF_FREQ * Eb_mod * Octave_3;
pub const E3: f32 = REF_FREQ * E_mod * Octave_3;
pub const F3: f32 = REF_FREQ * F_mod * Octave_3;
pub const Fs3: f32 = REF_FREQ * Fs_mod * Octave_3;
pub const G3: f32 = REF_FREQ * G_mod * Octave_3;
pub const Ab3: f32 = REF_FREQ * Ab_mod * Octave_3;
pub const A3: f32 = REF_FREQ * A_mod * Octave_3;
pub const Bb3: f32 = REF_FREQ * Bb_mod * Octave_3;
pub const B3: f32 = REF_FREQ * B_mod * Octave_3;

pub const C4: f32 = REF_FREQ * C_mod * Octave_4; 
pub const Db4: f32 = REF_FREQ * Db_mod * Octave_4;
pub const D4: f32 = REF_FREQ * D_mod * Octave_4;
pub const Eb4: f32 = REF_FREQ * Eb_mod * Octave_4;
pub const E4: f32 = REF_FREQ * E_mod * Octave_4;
pub const F4: f32 = REF_FREQ * F_mod * Octave_4;
pub const Fs4: f32 = REF_FREQ * Fs_mod * Octave_4;
pub const G4: f32 = REF_FREQ * G_mod * Octave_4;
pub const Ab4: f32 = REF_FREQ * Ab_mod * Octave_4;
pub const A4: f32 = REF_FREQ * A_mod * Octave_4;
pub const Bb4: f32 = REF_FREQ * Bb_mod * Octave_4;
pub const B4: f32 = REF_FREQ * B_mod * Octave_4;

pub const C5: f32 = REF_FREQ * C_mod * Octave_5;
pub const Db5: f32 = REF_FREQ * Db_mod * Octave_5;
pub const D5: f32 = REF_FREQ * D_mod * Octave_5;
pub const Eb5: f32 = REF_FREQ * Eb_mod * Octave_5;
pub const E5: f32 = REF_FREQ * E_mod * Octave_5;
pub const F5: f32 = REF_FREQ * F_mod * Octave_5;
pub const Fs5: f32 = REF_FREQ * Fs_mod * Octave_5;
pub const G5: f32 = REF_FREQ * G_mod * Octave_5;
pub const Ab5: f32 = REF_FREQ * Ab_mod * Octave_5;
pub const A5: f32 = REF_FREQ * A_mod * Octave_5;
pub const Bb5: f32 = REF_FREQ * Bb_mod * Octave_5;
pub const B5: f32 = REF_FREQ * B_mod * Octave_5;

pub const C6: f32 = REF_FREQ * C_mod * Octave_6;
pub const Db6: f32 = REF_FREQ * Db_mod * Octave_6;
pub const D6: f32 = REF_FREQ * D_mod * Octave_6;
pub const Eb6: f32 = REF_FREQ * Eb_mod * Octave_6;
pub const E6: f32 = REF_FREQ * E_mod * Octave_6;
pub const F6: f32 = REF_FREQ * F_mod * Octave_6;
pub const Fs6: f32 = REF_FREQ * Fs_mod * Octave_6;
pub const G6: f32 = REF_FREQ * G_mod * Octave_6;
pub const Ab6: f32 = REF_FREQ * Ab_mod * Octave_6;
pub const A6: f32 = REF_FREQ * A_mod * Octave_6;
pub const Bb6: f32 = REF_FREQ * Bb_mod * Octave_6;
pub const B6: f32 = REF_FREQ * B_mod * Octave_6;

pub const C7: f32 = REF_FREQ * C_mod * Octave_7;
pub const Db7: f32 = REF_FREQ * Db_mod * Octave_7;
pub const D7: f32 = REF_FREQ * D_mod * Octave_7;
pub const Eb7: f32 = REF_FREQ * Eb_mod * Octave_7;
pub const E7: f32 = REF_FREQ * E_mod * Octave_7;
pub const F7: f32 = REF_FREQ * F_mod * Octave_7;
pub const Fs7: f32 = REF_FREQ * Fs_mod * Octave_7;
pub const G7: f32 = REF_FREQ * G_mod * Octave_7;
pub const Ab7: f32 = REF_FREQ * Ab_mod * Octave_7;
pub const A7: f32 = REF_FREQ * A_mod * Octave_7;
pub const Bb7: f32 = REF_FREQ * Bb_mod * Octave_7;
pub const B7: f32 = REF_FREQ * B_mod * Octave_7;

pub const C8: f32 = REF_FREQ * C_mod * Octave_8;
pub const Db8: f32 = REF_FREQ * Db_mod * Octave_8;
pub const D8: f32 = REF_FREQ * D_mod * Octave_8;
pub const Eb8: f32 = REF_FREQ * Eb_mod * Octave_8;
pub const E8: f32 = REF_FREQ * E_mod * Octave_8;
pub const F8: f32 = REF_FREQ * F_mod * Octave_8;
pub const Fs8: f32 = REF_FREQ * Fs_mod * Octave_8;
pub const G8: f32 = REF_FREQ * G_mod * Octave_8;
pub const Ab8: f32 = REF_FREQ * Ab_mod * Octave_8;
pub const A8: f32 = REF_FREQ * A_mod * Octave_8;
pub const Bb8: f32 = REF_FREQ * Bb_mod * Octave_8;
pub const B8: f32 = REF_FREQ * B_mod * Octave_8;

pub const C9: f32 = REF_FREQ * C_mod * Octave_9;
pub const Db9: f32 = REF_FREQ * Db_mod * Octave_9;
pub const D9: f32 = REF_FREQ * D_mod * Octave_9;
pub const Eb9: f32 = REF_FREQ * Eb_mod * Octave_9;
pub const E9: f32 = REF_FREQ * E_mod * Octave_9;
pub const F9: f32 = REF_FREQ * F_mod * Octave_9;
pub const Fs9: f32 = REF_FREQ * Fs_mod * Octave_9;
pub const G9: f32 = REF_FREQ * G_mod * Octave_9;
pub const Ab9: f32 = REF_FREQ * Ab_mod * Octave_9;
pub const A9: f32 = REF_FREQ * A_mod * Octave_9;
pub const Bb9: f32 = REF_FREQ * Bb_mod * Octave_9;
pub const B9: f32 = REF_FREQ * B_mod * Octave_9;