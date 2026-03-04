use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ProofMessage {
    ok: bool,
    source: &'static str,
    checksum: u64,
}

#[derive(Serialize, Deserialize)]
struct ProofRecord0 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_0() -> u64 {
    let record = ProofRecord0 {
        id: 0u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1() -> u64 {
    let record = ProofRecord1 {
        id: 1u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord2 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_2() -> u64 {
    let record = ProofRecord2 {
        id: 2u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord3 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_3() -> u64 {
    let record = ProofRecord3 {
        id: 3u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord4 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_4() -> u64 {
    let record = ProofRecord4 {
        id: 4u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord5 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_5() -> u64 {
    let record = ProofRecord5 {
        id: 5u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord6 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_6() -> u64 {
    let record = ProofRecord6 {
        id: 6u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord7 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_7() -> u64 {
    let record = ProofRecord7 {
        id: 7u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord8 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_8() -> u64 {
    let record = ProofRecord8 {
        id: 8u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord9 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_9() -> u64 {
    let record = ProofRecord9 {
        id: 9u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord10 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_10() -> u64 {
    let record = ProofRecord10 {
        id: 10u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord11 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_11() -> u64 {
    let record = ProofRecord11 {
        id: 11u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord12 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_12() -> u64 {
    let record = ProofRecord12 {
        id: 12u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord13 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_13() -> u64 {
    let record = ProofRecord13 {
        id: 13u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord14 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_14() -> u64 {
    let record = ProofRecord14 {
        id: 14u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord15 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_15() -> u64 {
    let record = ProofRecord15 {
        id: 15u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord16 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_16() -> u64 {
    let record = ProofRecord16 {
        id: 16u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord17 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_17() -> u64 {
    let record = ProofRecord17 {
        id: 17u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord18 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_18() -> u64 {
    let record = ProofRecord18 {
        id: 18u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord19 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_19() -> u64 {
    let record = ProofRecord19 {
        id: 19u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord20 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_20() -> u64 {
    let record = ProofRecord20 {
        id: 20u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord21 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_21() -> u64 {
    let record = ProofRecord21 {
        id: 21u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord22 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_22() -> u64 {
    let record = ProofRecord22 {
        id: 22u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord23 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_23() -> u64 {
    let record = ProofRecord23 {
        id: 23u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord24 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_24() -> u64 {
    let record = ProofRecord24 {
        id: 24u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord25 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_25() -> u64 {
    let record = ProofRecord25 {
        id: 25u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord26 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_26() -> u64 {
    let record = ProofRecord26 {
        id: 26u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord27 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_27() -> u64 {
    let record = ProofRecord27 {
        id: 27u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord28 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_28() -> u64 {
    let record = ProofRecord28 {
        id: 28u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord29 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_29() -> u64 {
    let record = ProofRecord29 {
        id: 29u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord30 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_30() -> u64 {
    let record = ProofRecord30 {
        id: 30u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord31 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_31() -> u64 {
    let record = ProofRecord31 {
        id: 31u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord32 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_32() -> u64 {
    let record = ProofRecord32 {
        id: 32u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord33 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_33() -> u64 {
    let record = ProofRecord33 {
        id: 33u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord34 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_34() -> u64 {
    let record = ProofRecord34 {
        id: 34u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord35 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_35() -> u64 {
    let record = ProofRecord35 {
        id: 35u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord36 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_36() -> u64 {
    let record = ProofRecord36 {
        id: 36u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord37 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_37() -> u64 {
    let record = ProofRecord37 {
        id: 37u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord38 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_38() -> u64 {
    let record = ProofRecord38 {
        id: 38u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord39 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_39() -> u64 {
    let record = ProofRecord39 {
        id: 39u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord40 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_40() -> u64 {
    let record = ProofRecord40 {
        id: 40u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord41 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_41() -> u64 {
    let record = ProofRecord41 {
        id: 41u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord42 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_42() -> u64 {
    let record = ProofRecord42 {
        id: 42u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord43 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_43() -> u64 {
    let record = ProofRecord43 {
        id: 43u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord44 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_44() -> u64 {
    let record = ProofRecord44 {
        id: 44u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord45 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_45() -> u64 {
    let record = ProofRecord45 {
        id: 45u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord46 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_46() -> u64 {
    let record = ProofRecord46 {
        id: 46u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord47 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_47() -> u64 {
    let record = ProofRecord47 {
        id: 47u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord48 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_48() -> u64 {
    let record = ProofRecord48 {
        id: 48u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord49 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_49() -> u64 {
    let record = ProofRecord49 {
        id: 49u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord50 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_50() -> u64 {
    let record = ProofRecord50 {
        id: 50u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord51 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_51() -> u64 {
    let record = ProofRecord51 {
        id: 51u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord52 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_52() -> u64 {
    let record = ProofRecord52 {
        id: 52u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord53 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_53() -> u64 {
    let record = ProofRecord53 {
        id: 53u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord54 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_54() -> u64 {
    let record = ProofRecord54 {
        id: 54u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord55 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_55() -> u64 {
    let record = ProofRecord55 {
        id: 55u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord56 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_56() -> u64 {
    let record = ProofRecord56 {
        id: 56u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord57 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_57() -> u64 {
    let record = ProofRecord57 {
        id: 57u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord58 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_58() -> u64 {
    let record = ProofRecord58 {
        id: 58u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord59 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_59() -> u64 {
    let record = ProofRecord59 {
        id: 59u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord60 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_60() -> u64 {
    let record = ProofRecord60 {
        id: 60u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord61 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_61() -> u64 {
    let record = ProofRecord61 {
        id: 61u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord62 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_62() -> u64 {
    let record = ProofRecord62 {
        id: 62u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord63 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_63() -> u64 {
    let record = ProofRecord63 {
        id: 63u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord64 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_64() -> u64 {
    let record = ProofRecord64 {
        id: 64u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord65 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_65() -> u64 {
    let record = ProofRecord65 {
        id: 65u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord66 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_66() -> u64 {
    let record = ProofRecord66 {
        id: 66u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord67 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_67() -> u64 {
    let record = ProofRecord67 {
        id: 67u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord68 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_68() -> u64 {
    let record = ProofRecord68 {
        id: 68u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord69 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_69() -> u64 {
    let record = ProofRecord69 {
        id: 69u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord70 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_70() -> u64 {
    let record = ProofRecord70 {
        id: 70u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord71 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_71() -> u64 {
    let record = ProofRecord71 {
        id: 71u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord72 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_72() -> u64 {
    let record = ProofRecord72 {
        id: 72u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord73 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_73() -> u64 {
    let record = ProofRecord73 {
        id: 73u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord74 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_74() -> u64 {
    let record = ProofRecord74 {
        id: 74u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord75 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_75() -> u64 {
    let record = ProofRecord75 {
        id: 75u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord76 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_76() -> u64 {
    let record = ProofRecord76 {
        id: 76u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord77 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_77() -> u64 {
    let record = ProofRecord77 {
        id: 77u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord78 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_78() -> u64 {
    let record = ProofRecord78 {
        id: 78u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord79 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_79() -> u64 {
    let record = ProofRecord79 {
        id: 79u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord80 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_80() -> u64 {
    let record = ProofRecord80 {
        id: 80u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord81 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_81() -> u64 {
    let record = ProofRecord81 {
        id: 81u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord82 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_82() -> u64 {
    let record = ProofRecord82 {
        id: 82u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord83 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_83() -> u64 {
    let record = ProofRecord83 {
        id: 83u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord84 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_84() -> u64 {
    let record = ProofRecord84 {
        id: 84u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord85 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_85() -> u64 {
    let record = ProofRecord85 {
        id: 85u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord86 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_86() -> u64 {
    let record = ProofRecord86 {
        id: 86u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord87 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_87() -> u64 {
    let record = ProofRecord87 {
        id: 87u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord88 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_88() -> u64 {
    let record = ProofRecord88 {
        id: 88u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord89 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_89() -> u64 {
    let record = ProofRecord89 {
        id: 89u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord90 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_90() -> u64 {
    let record = ProofRecord90 {
        id: 90u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord91 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_91() -> u64 {
    let record = ProofRecord91 {
        id: 91u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord92 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_92() -> u64 {
    let record = ProofRecord92 {
        id: 92u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord93 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_93() -> u64 {
    let record = ProofRecord93 {
        id: 93u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord94 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_94() -> u64 {
    let record = ProofRecord94 {
        id: 94u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord95 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_95() -> u64 {
    let record = ProofRecord95 {
        id: 95u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord96 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_96() -> u64 {
    let record = ProofRecord96 {
        id: 96u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord97 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_97() -> u64 {
    let record = ProofRecord97 {
        id: 97u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord98 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_98() -> u64 {
    let record = ProofRecord98 {
        id: 98u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord99 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_99() -> u64 {
    let record = ProofRecord99 {
        id: 99u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord100 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_100() -> u64 {
    let record = ProofRecord100 {
        id: 100u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord101 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_101() -> u64 {
    let record = ProofRecord101 {
        id: 101u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord102 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_102() -> u64 {
    let record = ProofRecord102 {
        id: 102u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord103 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_103() -> u64 {
    let record = ProofRecord103 {
        id: 103u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord104 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_104() -> u64 {
    let record = ProofRecord104 {
        id: 104u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord105 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_105() -> u64 {
    let record = ProofRecord105 {
        id: 105u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord106 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_106() -> u64 {
    let record = ProofRecord106 {
        id: 106u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord107 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_107() -> u64 {
    let record = ProofRecord107 {
        id: 107u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord108 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_108() -> u64 {
    let record = ProofRecord108 {
        id: 108u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord109 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_109() -> u64 {
    let record = ProofRecord109 {
        id: 109u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord110 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_110() -> u64 {
    let record = ProofRecord110 {
        id: 110u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord111 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_111() -> u64 {
    let record = ProofRecord111 {
        id: 111u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord112 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_112() -> u64 {
    let record = ProofRecord112 {
        id: 112u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord113 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_113() -> u64 {
    let record = ProofRecord113 {
        id: 113u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord114 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_114() -> u64 {
    let record = ProofRecord114 {
        id: 114u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord115 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_115() -> u64 {
    let record = ProofRecord115 {
        id: 115u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord116 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_116() -> u64 {
    let record = ProofRecord116 {
        id: 116u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord117 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_117() -> u64 {
    let record = ProofRecord117 {
        id: 117u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord118 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_118() -> u64 {
    let record = ProofRecord118 {
        id: 118u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord119 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_119() -> u64 {
    let record = ProofRecord119 {
        id: 119u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord120 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_120() -> u64 {
    let record = ProofRecord120 {
        id: 120u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord121 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_121() -> u64 {
    let record = ProofRecord121 {
        id: 121u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord122 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_122() -> u64 {
    let record = ProofRecord122 {
        id: 122u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord123 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_123() -> u64 {
    let record = ProofRecord123 {
        id: 123u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord124 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_124() -> u64 {
    let record = ProofRecord124 {
        id: 124u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord125 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_125() -> u64 {
    let record = ProofRecord125 {
        id: 125u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord126 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_126() -> u64 {
    let record = ProofRecord126 {
        id: 126u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord127 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_127() -> u64 {
    let record = ProofRecord127 {
        id: 127u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord128 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_128() -> u64 {
    let record = ProofRecord128 {
        id: 128u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord129 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_129() -> u64 {
    let record = ProofRecord129 {
        id: 129u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord130 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_130() -> u64 {
    let record = ProofRecord130 {
        id: 130u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord131 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_131() -> u64 {
    let record = ProofRecord131 {
        id: 131u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord132 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_132() -> u64 {
    let record = ProofRecord132 {
        id: 132u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord133 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_133() -> u64 {
    let record = ProofRecord133 {
        id: 133u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord134 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_134() -> u64 {
    let record = ProofRecord134 {
        id: 134u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord135 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_135() -> u64 {
    let record = ProofRecord135 {
        id: 135u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord136 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_136() -> u64 {
    let record = ProofRecord136 {
        id: 136u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord137 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_137() -> u64 {
    let record = ProofRecord137 {
        id: 137u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord138 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_138() -> u64 {
    let record = ProofRecord138 {
        id: 138u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord139 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_139() -> u64 {
    let record = ProofRecord139 {
        id: 139u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord140 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_140() -> u64 {
    let record = ProofRecord140 {
        id: 140u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord141 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_141() -> u64 {
    let record = ProofRecord141 {
        id: 141u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord142 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_142() -> u64 {
    let record = ProofRecord142 {
        id: 142u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord143 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_143() -> u64 {
    let record = ProofRecord143 {
        id: 143u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord144 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_144() -> u64 {
    let record = ProofRecord144 {
        id: 144u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord145 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_145() -> u64 {
    let record = ProofRecord145 {
        id: 145u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord146 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_146() -> u64 {
    let record = ProofRecord146 {
        id: 146u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord147 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_147() -> u64 {
    let record = ProofRecord147 {
        id: 147u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord148 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_148() -> u64 {
    let record = ProofRecord148 {
        id: 148u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord149 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_149() -> u64 {
    let record = ProofRecord149 {
        id: 149u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord150 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_150() -> u64 {
    let record = ProofRecord150 {
        id: 150u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord151 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_151() -> u64 {
    let record = ProofRecord151 {
        id: 151u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord152 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_152() -> u64 {
    let record = ProofRecord152 {
        id: 152u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord153 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_153() -> u64 {
    let record = ProofRecord153 {
        id: 153u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord154 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_154() -> u64 {
    let record = ProofRecord154 {
        id: 154u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord155 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_155() -> u64 {
    let record = ProofRecord155 {
        id: 155u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord156 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_156() -> u64 {
    let record = ProofRecord156 {
        id: 156u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord157 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_157() -> u64 {
    let record = ProofRecord157 {
        id: 157u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord158 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_158() -> u64 {
    let record = ProofRecord158 {
        id: 158u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord159 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_159() -> u64 {
    let record = ProofRecord159 {
        id: 159u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord160 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_160() -> u64 {
    let record = ProofRecord160 {
        id: 160u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord161 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_161() -> u64 {
    let record = ProofRecord161 {
        id: 161u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord162 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_162() -> u64 {
    let record = ProofRecord162 {
        id: 162u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord163 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_163() -> u64 {
    let record = ProofRecord163 {
        id: 163u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord164 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_164() -> u64 {
    let record = ProofRecord164 {
        id: 164u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord165 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_165() -> u64 {
    let record = ProofRecord165 {
        id: 165u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord166 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_166() -> u64 {
    let record = ProofRecord166 {
        id: 166u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord167 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_167() -> u64 {
    let record = ProofRecord167 {
        id: 167u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord168 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_168() -> u64 {
    let record = ProofRecord168 {
        id: 168u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord169 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_169() -> u64 {
    let record = ProofRecord169 {
        id: 169u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord170 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_170() -> u64 {
    let record = ProofRecord170 {
        id: 170u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord171 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_171() -> u64 {
    let record = ProofRecord171 {
        id: 171u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord172 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_172() -> u64 {
    let record = ProofRecord172 {
        id: 172u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord173 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_173() -> u64 {
    let record = ProofRecord173 {
        id: 173u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord174 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_174() -> u64 {
    let record = ProofRecord174 {
        id: 174u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord175 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_175() -> u64 {
    let record = ProofRecord175 {
        id: 175u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord176 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_176() -> u64 {
    let record = ProofRecord176 {
        id: 176u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord177 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_177() -> u64 {
    let record = ProofRecord177 {
        id: 177u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord178 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_178() -> u64 {
    let record = ProofRecord178 {
        id: 178u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord179 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_179() -> u64 {
    let record = ProofRecord179 {
        id: 179u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord180 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_180() -> u64 {
    let record = ProofRecord180 {
        id: 180u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord181 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_181() -> u64 {
    let record = ProofRecord181 {
        id: 181u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord182 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_182() -> u64 {
    let record = ProofRecord182 {
        id: 182u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord183 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_183() -> u64 {
    let record = ProofRecord183 {
        id: 183u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord184 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_184() -> u64 {
    let record = ProofRecord184 {
        id: 184u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord185 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_185() -> u64 {
    let record = ProofRecord185 {
        id: 185u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord186 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_186() -> u64 {
    let record = ProofRecord186 {
        id: 186u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord187 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_187() -> u64 {
    let record = ProofRecord187 {
        id: 187u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord188 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_188() -> u64 {
    let record = ProofRecord188 {
        id: 188u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord189 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_189() -> u64 {
    let record = ProofRecord189 {
        id: 189u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord190 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_190() -> u64 {
    let record = ProofRecord190 {
        id: 190u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord191 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_191() -> u64 {
    let record = ProofRecord191 {
        id: 191u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord192 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_192() -> u64 {
    let record = ProofRecord192 {
        id: 192u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord193 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_193() -> u64 {
    let record = ProofRecord193 {
        id: 193u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord194 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_194() -> u64 {
    let record = ProofRecord194 {
        id: 194u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord195 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_195() -> u64 {
    let record = ProofRecord195 {
        id: 195u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord196 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_196() -> u64 {
    let record = ProofRecord196 {
        id: 196u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord197 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_197() -> u64 {
    let record = ProofRecord197 {
        id: 197u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord198 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_198() -> u64 {
    let record = ProofRecord198 {
        id: 198u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord199 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_199() -> u64 {
    let record = ProofRecord199 {
        id: 199u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord200 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_200() -> u64 {
    let record = ProofRecord200 {
        id: 200u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord201 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_201() -> u64 {
    let record = ProofRecord201 {
        id: 201u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord202 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_202() -> u64 {
    let record = ProofRecord202 {
        id: 202u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord203 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_203() -> u64 {
    let record = ProofRecord203 {
        id: 203u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord204 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_204() -> u64 {
    let record = ProofRecord204 {
        id: 204u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord205 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_205() -> u64 {
    let record = ProofRecord205 {
        id: 205u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord206 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_206() -> u64 {
    let record = ProofRecord206 {
        id: 206u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord207 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_207() -> u64 {
    let record = ProofRecord207 {
        id: 207u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord208 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_208() -> u64 {
    let record = ProofRecord208 {
        id: 208u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord209 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_209() -> u64 {
    let record = ProofRecord209 {
        id: 209u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord210 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_210() -> u64 {
    let record = ProofRecord210 {
        id: 210u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord211 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_211() -> u64 {
    let record = ProofRecord211 {
        id: 211u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord212 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_212() -> u64 {
    let record = ProofRecord212 {
        id: 212u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord213 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_213() -> u64 {
    let record = ProofRecord213 {
        id: 213u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord214 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_214() -> u64 {
    let record = ProofRecord214 {
        id: 214u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord215 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_215() -> u64 {
    let record = ProofRecord215 {
        id: 215u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord216 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_216() -> u64 {
    let record = ProofRecord216 {
        id: 216u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord217 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_217() -> u64 {
    let record = ProofRecord217 {
        id: 217u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord218 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_218() -> u64 {
    let record = ProofRecord218 {
        id: 218u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord219 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_219() -> u64 {
    let record = ProofRecord219 {
        id: 219u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord220 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_220() -> u64 {
    let record = ProofRecord220 {
        id: 220u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord221 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_221() -> u64 {
    let record = ProofRecord221 {
        id: 221u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord222 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_222() -> u64 {
    let record = ProofRecord222 {
        id: 222u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord223 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_223() -> u64 {
    let record = ProofRecord223 {
        id: 223u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord224 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_224() -> u64 {
    let record = ProofRecord224 {
        id: 224u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord225 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_225() -> u64 {
    let record = ProofRecord225 {
        id: 225u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord226 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_226() -> u64 {
    let record = ProofRecord226 {
        id: 226u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord227 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_227() -> u64 {
    let record = ProofRecord227 {
        id: 227u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord228 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_228() -> u64 {
    let record = ProofRecord228 {
        id: 228u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord229 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_229() -> u64 {
    let record = ProofRecord229 {
        id: 229u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord230 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_230() -> u64 {
    let record = ProofRecord230 {
        id: 230u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord231 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_231() -> u64 {
    let record = ProofRecord231 {
        id: 231u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord232 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_232() -> u64 {
    let record = ProofRecord232 {
        id: 232u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord233 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_233() -> u64 {
    let record = ProofRecord233 {
        id: 233u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord234 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_234() -> u64 {
    let record = ProofRecord234 {
        id: 234u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord235 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_235() -> u64 {
    let record = ProofRecord235 {
        id: 235u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord236 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_236() -> u64 {
    let record = ProofRecord236 {
        id: 236u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord237 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_237() -> u64 {
    let record = ProofRecord237 {
        id: 237u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord238 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_238() -> u64 {
    let record = ProofRecord238 {
        id: 238u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord239 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_239() -> u64 {
    let record = ProofRecord239 {
        id: 239u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord240 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_240() -> u64 {
    let record = ProofRecord240 {
        id: 240u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord241 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_241() -> u64 {
    let record = ProofRecord241 {
        id: 241u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord242 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_242() -> u64 {
    let record = ProofRecord242 {
        id: 242u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord243 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_243() -> u64 {
    let record = ProofRecord243 {
        id: 243u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord244 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_244() -> u64 {
    let record = ProofRecord244 {
        id: 244u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord245 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_245() -> u64 {
    let record = ProofRecord245 {
        id: 245u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord246 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_246() -> u64 {
    let record = ProofRecord246 {
        id: 246u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord247 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_247() -> u64 {
    let record = ProofRecord247 {
        id: 247u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord248 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_248() -> u64 {
    let record = ProofRecord248 {
        id: 248u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord249 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_249() -> u64 {
    let record = ProofRecord249 {
        id: 249u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord250 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_250() -> u64 {
    let record = ProofRecord250 {
        id: 250u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord251 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_251() -> u64 {
    let record = ProofRecord251 {
        id: 251u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord252 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_252() -> u64 {
    let record = ProofRecord252 {
        id: 252u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord253 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_253() -> u64 {
    let record = ProofRecord253 {
        id: 253u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord254 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_254() -> u64 {
    let record = ProofRecord254 {
        id: 254u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord255 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_255() -> u64 {
    let record = ProofRecord255 {
        id: 255u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord256 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_256() -> u64 {
    let record = ProofRecord256 {
        id: 256u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord257 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_257() -> u64 {
    let record = ProofRecord257 {
        id: 257u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord258 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_258() -> u64 {
    let record = ProofRecord258 {
        id: 258u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord259 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_259() -> u64 {
    let record = ProofRecord259 {
        id: 259u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord260 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_260() -> u64 {
    let record = ProofRecord260 {
        id: 260u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord261 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_261() -> u64 {
    let record = ProofRecord261 {
        id: 261u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord262 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_262() -> u64 {
    let record = ProofRecord262 {
        id: 262u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord263 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_263() -> u64 {
    let record = ProofRecord263 {
        id: 263u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord264 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_264() -> u64 {
    let record = ProofRecord264 {
        id: 264u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord265 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_265() -> u64 {
    let record = ProofRecord265 {
        id: 265u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord266 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_266() -> u64 {
    let record = ProofRecord266 {
        id: 266u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord267 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_267() -> u64 {
    let record = ProofRecord267 {
        id: 267u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord268 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_268() -> u64 {
    let record = ProofRecord268 {
        id: 268u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord269 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_269() -> u64 {
    let record = ProofRecord269 {
        id: 269u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord270 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_270() -> u64 {
    let record = ProofRecord270 {
        id: 270u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord271 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_271() -> u64 {
    let record = ProofRecord271 {
        id: 271u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord272 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_272() -> u64 {
    let record = ProofRecord272 {
        id: 272u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord273 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_273() -> u64 {
    let record = ProofRecord273 {
        id: 273u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord274 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_274() -> u64 {
    let record = ProofRecord274 {
        id: 274u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord275 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_275() -> u64 {
    let record = ProofRecord275 {
        id: 275u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord276 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_276() -> u64 {
    let record = ProofRecord276 {
        id: 276u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord277 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_277() -> u64 {
    let record = ProofRecord277 {
        id: 277u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord278 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_278() -> u64 {
    let record = ProofRecord278 {
        id: 278u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord279 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_279() -> u64 {
    let record = ProofRecord279 {
        id: 279u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord280 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_280() -> u64 {
    let record = ProofRecord280 {
        id: 280u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord281 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_281() -> u64 {
    let record = ProofRecord281 {
        id: 281u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord282 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_282() -> u64 {
    let record = ProofRecord282 {
        id: 282u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord283 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_283() -> u64 {
    let record = ProofRecord283 {
        id: 283u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord284 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_284() -> u64 {
    let record = ProofRecord284 {
        id: 284u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord285 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_285() -> u64 {
    let record = ProofRecord285 {
        id: 285u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord286 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_286() -> u64 {
    let record = ProofRecord286 {
        id: 286u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord287 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_287() -> u64 {
    let record = ProofRecord287 {
        id: 287u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord288 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_288() -> u64 {
    let record = ProofRecord288 {
        id: 288u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord289 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_289() -> u64 {
    let record = ProofRecord289 {
        id: 289u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord290 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_290() -> u64 {
    let record = ProofRecord290 {
        id: 290u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord291 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_291() -> u64 {
    let record = ProofRecord291 {
        id: 291u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord292 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_292() -> u64 {
    let record = ProofRecord292 {
        id: 292u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord293 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_293() -> u64 {
    let record = ProofRecord293 {
        id: 293u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord294 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_294() -> u64 {
    let record = ProofRecord294 {
        id: 294u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord295 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_295() -> u64 {
    let record = ProofRecord295 {
        id: 295u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord296 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_296() -> u64 {
    let record = ProofRecord296 {
        id: 296u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord297 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_297() -> u64 {
    let record = ProofRecord297 {
        id: 297u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord298 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_298() -> u64 {
    let record = ProofRecord298 {
        id: 298u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord299 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_299() -> u64 {
    let record = ProofRecord299 {
        id: 299u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord300 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_300() -> u64 {
    let record = ProofRecord300 {
        id: 300u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord301 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_301() -> u64 {
    let record = ProofRecord301 {
        id: 301u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord302 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_302() -> u64 {
    let record = ProofRecord302 {
        id: 302u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord303 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_303() -> u64 {
    let record = ProofRecord303 {
        id: 303u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord304 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_304() -> u64 {
    let record = ProofRecord304 {
        id: 304u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord305 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_305() -> u64 {
    let record = ProofRecord305 {
        id: 305u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord306 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_306() -> u64 {
    let record = ProofRecord306 {
        id: 306u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord307 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_307() -> u64 {
    let record = ProofRecord307 {
        id: 307u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord308 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_308() -> u64 {
    let record = ProofRecord308 {
        id: 308u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord309 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_309() -> u64 {
    let record = ProofRecord309 {
        id: 309u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord310 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_310() -> u64 {
    let record = ProofRecord310 {
        id: 310u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord311 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_311() -> u64 {
    let record = ProofRecord311 {
        id: 311u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord312 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_312() -> u64 {
    let record = ProofRecord312 {
        id: 312u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord313 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_313() -> u64 {
    let record = ProofRecord313 {
        id: 313u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord314 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_314() -> u64 {
    let record = ProofRecord314 {
        id: 314u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord315 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_315() -> u64 {
    let record = ProofRecord315 {
        id: 315u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord316 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_316() -> u64 {
    let record = ProofRecord316 {
        id: 316u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord317 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_317() -> u64 {
    let record = ProofRecord317 {
        id: 317u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord318 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_318() -> u64 {
    let record = ProofRecord318 {
        id: 318u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord319 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_319() -> u64 {
    let record = ProofRecord319 {
        id: 319u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord320 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_320() -> u64 {
    let record = ProofRecord320 {
        id: 320u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord321 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_321() -> u64 {
    let record = ProofRecord321 {
        id: 321u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord322 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_322() -> u64 {
    let record = ProofRecord322 {
        id: 322u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord323 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_323() -> u64 {
    let record = ProofRecord323 {
        id: 323u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord324 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_324() -> u64 {
    let record = ProofRecord324 {
        id: 324u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord325 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_325() -> u64 {
    let record = ProofRecord325 {
        id: 325u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord326 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_326() -> u64 {
    let record = ProofRecord326 {
        id: 326u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord327 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_327() -> u64 {
    let record = ProofRecord327 {
        id: 327u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord328 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_328() -> u64 {
    let record = ProofRecord328 {
        id: 328u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord329 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_329() -> u64 {
    let record = ProofRecord329 {
        id: 329u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord330 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_330() -> u64 {
    let record = ProofRecord330 {
        id: 330u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord331 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_331() -> u64 {
    let record = ProofRecord331 {
        id: 331u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord332 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_332() -> u64 {
    let record = ProofRecord332 {
        id: 332u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord333 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_333() -> u64 {
    let record = ProofRecord333 {
        id: 333u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord334 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_334() -> u64 {
    let record = ProofRecord334 {
        id: 334u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord335 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_335() -> u64 {
    let record = ProofRecord335 {
        id: 335u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord336 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_336() -> u64 {
    let record = ProofRecord336 {
        id: 336u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord337 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_337() -> u64 {
    let record = ProofRecord337 {
        id: 337u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord338 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_338() -> u64 {
    let record = ProofRecord338 {
        id: 338u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord339 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_339() -> u64 {
    let record = ProofRecord339 {
        id: 339u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord340 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_340() -> u64 {
    let record = ProofRecord340 {
        id: 340u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord341 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_341() -> u64 {
    let record = ProofRecord341 {
        id: 341u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord342 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_342() -> u64 {
    let record = ProofRecord342 {
        id: 342u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord343 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_343() -> u64 {
    let record = ProofRecord343 {
        id: 343u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord344 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_344() -> u64 {
    let record = ProofRecord344 {
        id: 344u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord345 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_345() -> u64 {
    let record = ProofRecord345 {
        id: 345u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord346 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_346() -> u64 {
    let record = ProofRecord346 {
        id: 346u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord347 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_347() -> u64 {
    let record = ProofRecord347 {
        id: 347u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord348 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_348() -> u64 {
    let record = ProofRecord348 {
        id: 348u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord349 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_349() -> u64 {
    let record = ProofRecord349 {
        id: 349u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord350 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_350() -> u64 {
    let record = ProofRecord350 {
        id: 350u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord351 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_351() -> u64 {
    let record = ProofRecord351 {
        id: 351u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord352 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_352() -> u64 {
    let record = ProofRecord352 {
        id: 352u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord353 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_353() -> u64 {
    let record = ProofRecord353 {
        id: 353u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord354 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_354() -> u64 {
    let record = ProofRecord354 {
        id: 354u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord355 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_355() -> u64 {
    let record = ProofRecord355 {
        id: 355u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord356 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_356() -> u64 {
    let record = ProofRecord356 {
        id: 356u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord357 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_357() -> u64 {
    let record = ProofRecord357 {
        id: 357u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord358 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_358() -> u64 {
    let record = ProofRecord358 {
        id: 358u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord359 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_359() -> u64 {
    let record = ProofRecord359 {
        id: 359u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord360 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_360() -> u64 {
    let record = ProofRecord360 {
        id: 360u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord361 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_361() -> u64 {
    let record = ProofRecord361 {
        id: 361u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord362 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_362() -> u64 {
    let record = ProofRecord362 {
        id: 362u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord363 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_363() -> u64 {
    let record = ProofRecord363 {
        id: 363u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord364 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_364() -> u64 {
    let record = ProofRecord364 {
        id: 364u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord365 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_365() -> u64 {
    let record = ProofRecord365 {
        id: 365u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord366 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_366() -> u64 {
    let record = ProofRecord366 {
        id: 366u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord367 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_367() -> u64 {
    let record = ProofRecord367 {
        id: 367u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord368 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_368() -> u64 {
    let record = ProofRecord368 {
        id: 368u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord369 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_369() -> u64 {
    let record = ProofRecord369 {
        id: 369u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord370 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_370() -> u64 {
    let record = ProofRecord370 {
        id: 370u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord371 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_371() -> u64 {
    let record = ProofRecord371 {
        id: 371u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord372 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_372() -> u64 {
    let record = ProofRecord372 {
        id: 372u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord373 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_373() -> u64 {
    let record = ProofRecord373 {
        id: 373u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord374 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_374() -> u64 {
    let record = ProofRecord374 {
        id: 374u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord375 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_375() -> u64 {
    let record = ProofRecord375 {
        id: 375u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord376 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_376() -> u64 {
    let record = ProofRecord376 {
        id: 376u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord377 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_377() -> u64 {
    let record = ProofRecord377 {
        id: 377u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord378 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_378() -> u64 {
    let record = ProofRecord378 {
        id: 378u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord379 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_379() -> u64 {
    let record = ProofRecord379 {
        id: 379u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord380 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_380() -> u64 {
    let record = ProofRecord380 {
        id: 380u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord381 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_381() -> u64 {
    let record = ProofRecord381 {
        id: 381u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord382 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_382() -> u64 {
    let record = ProofRecord382 {
        id: 382u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord383 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_383() -> u64 {
    let record = ProofRecord383 {
        id: 383u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord384 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_384() -> u64 {
    let record = ProofRecord384 {
        id: 384u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord385 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_385() -> u64 {
    let record = ProofRecord385 {
        id: 385u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord386 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_386() -> u64 {
    let record = ProofRecord386 {
        id: 386u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord387 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_387() -> u64 {
    let record = ProofRecord387 {
        id: 387u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord388 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_388() -> u64 {
    let record = ProofRecord388 {
        id: 388u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord389 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_389() -> u64 {
    let record = ProofRecord389 {
        id: 389u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord390 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_390() -> u64 {
    let record = ProofRecord390 {
        id: 390u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord391 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_391() -> u64 {
    let record = ProofRecord391 {
        id: 391u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord392 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_392() -> u64 {
    let record = ProofRecord392 {
        id: 392u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord393 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_393() -> u64 {
    let record = ProofRecord393 {
        id: 393u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord394 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_394() -> u64 {
    let record = ProofRecord394 {
        id: 394u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord395 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_395() -> u64 {
    let record = ProofRecord395 {
        id: 395u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord396 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_396() -> u64 {
    let record = ProofRecord396 {
        id: 396u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord397 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_397() -> u64 {
    let record = ProofRecord397 {
        id: 397u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord398 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_398() -> u64 {
    let record = ProofRecord398 {
        id: 398u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord399 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_399() -> u64 {
    let record = ProofRecord399 {
        id: 399u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord400 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_400() -> u64 {
    let record = ProofRecord400 {
        id: 400u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord401 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_401() -> u64 {
    let record = ProofRecord401 {
        id: 401u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord402 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_402() -> u64 {
    let record = ProofRecord402 {
        id: 402u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord403 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_403() -> u64 {
    let record = ProofRecord403 {
        id: 403u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord404 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_404() -> u64 {
    let record = ProofRecord404 {
        id: 404u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord405 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_405() -> u64 {
    let record = ProofRecord405 {
        id: 405u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord406 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_406() -> u64 {
    let record = ProofRecord406 {
        id: 406u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord407 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_407() -> u64 {
    let record = ProofRecord407 {
        id: 407u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord408 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_408() -> u64 {
    let record = ProofRecord408 {
        id: 408u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord409 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_409() -> u64 {
    let record = ProofRecord409 {
        id: 409u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord410 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_410() -> u64 {
    let record = ProofRecord410 {
        id: 410u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord411 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_411() -> u64 {
    let record = ProofRecord411 {
        id: 411u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord412 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_412() -> u64 {
    let record = ProofRecord412 {
        id: 412u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord413 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_413() -> u64 {
    let record = ProofRecord413 {
        id: 413u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord414 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_414() -> u64 {
    let record = ProofRecord414 {
        id: 414u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord415 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_415() -> u64 {
    let record = ProofRecord415 {
        id: 415u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord416 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_416() -> u64 {
    let record = ProofRecord416 {
        id: 416u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord417 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_417() -> u64 {
    let record = ProofRecord417 {
        id: 417u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord418 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_418() -> u64 {
    let record = ProofRecord418 {
        id: 418u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord419 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_419() -> u64 {
    let record = ProofRecord419 {
        id: 419u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord420 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_420() -> u64 {
    let record = ProofRecord420 {
        id: 420u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord421 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_421() -> u64 {
    let record = ProofRecord421 {
        id: 421u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord422 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_422() -> u64 {
    let record = ProofRecord422 {
        id: 422u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord423 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_423() -> u64 {
    let record = ProofRecord423 {
        id: 423u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord424 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_424() -> u64 {
    let record = ProofRecord424 {
        id: 424u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord425 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_425() -> u64 {
    let record = ProofRecord425 {
        id: 425u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord426 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_426() -> u64 {
    let record = ProofRecord426 {
        id: 426u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord427 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_427() -> u64 {
    let record = ProofRecord427 {
        id: 427u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord428 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_428() -> u64 {
    let record = ProofRecord428 {
        id: 428u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord429 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_429() -> u64 {
    let record = ProofRecord429 {
        id: 429u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord430 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_430() -> u64 {
    let record = ProofRecord430 {
        id: 430u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord431 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_431() -> u64 {
    let record = ProofRecord431 {
        id: 431u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord432 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_432() -> u64 {
    let record = ProofRecord432 {
        id: 432u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord433 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_433() -> u64 {
    let record = ProofRecord433 {
        id: 433u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord434 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_434() -> u64 {
    let record = ProofRecord434 {
        id: 434u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord435 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_435() -> u64 {
    let record = ProofRecord435 {
        id: 435u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord436 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_436() -> u64 {
    let record = ProofRecord436 {
        id: 436u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord437 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_437() -> u64 {
    let record = ProofRecord437 {
        id: 437u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord438 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_438() -> u64 {
    let record = ProofRecord438 {
        id: 438u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord439 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_439() -> u64 {
    let record = ProofRecord439 {
        id: 439u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord440 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_440() -> u64 {
    let record = ProofRecord440 {
        id: 440u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord441 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_441() -> u64 {
    let record = ProofRecord441 {
        id: 441u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord442 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_442() -> u64 {
    let record = ProofRecord442 {
        id: 442u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord443 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_443() -> u64 {
    let record = ProofRecord443 {
        id: 443u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord444 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_444() -> u64 {
    let record = ProofRecord444 {
        id: 444u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord445 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_445() -> u64 {
    let record = ProofRecord445 {
        id: 445u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord446 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_446() -> u64 {
    let record = ProofRecord446 {
        id: 446u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord447 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_447() -> u64 {
    let record = ProofRecord447 {
        id: 447u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord448 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_448() -> u64 {
    let record = ProofRecord448 {
        id: 448u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord449 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_449() -> u64 {
    let record = ProofRecord449 {
        id: 449u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord450 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_450() -> u64 {
    let record = ProofRecord450 {
        id: 450u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord451 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_451() -> u64 {
    let record = ProofRecord451 {
        id: 451u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord452 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_452() -> u64 {
    let record = ProofRecord452 {
        id: 452u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord453 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_453() -> u64 {
    let record = ProofRecord453 {
        id: 453u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord454 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_454() -> u64 {
    let record = ProofRecord454 {
        id: 454u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord455 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_455() -> u64 {
    let record = ProofRecord455 {
        id: 455u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord456 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_456() -> u64 {
    let record = ProofRecord456 {
        id: 456u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord457 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_457() -> u64 {
    let record = ProofRecord457 {
        id: 457u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord458 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_458() -> u64 {
    let record = ProofRecord458 {
        id: 458u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord459 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_459() -> u64 {
    let record = ProofRecord459 {
        id: 459u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord460 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_460() -> u64 {
    let record = ProofRecord460 {
        id: 460u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord461 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_461() -> u64 {
    let record = ProofRecord461 {
        id: 461u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord462 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_462() -> u64 {
    let record = ProofRecord462 {
        id: 462u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord463 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_463() -> u64 {
    let record = ProofRecord463 {
        id: 463u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord464 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_464() -> u64 {
    let record = ProofRecord464 {
        id: 464u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord465 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_465() -> u64 {
    let record = ProofRecord465 {
        id: 465u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord466 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_466() -> u64 {
    let record = ProofRecord466 {
        id: 466u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord467 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_467() -> u64 {
    let record = ProofRecord467 {
        id: 467u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord468 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_468() -> u64 {
    let record = ProofRecord468 {
        id: 468u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord469 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_469() -> u64 {
    let record = ProofRecord469 {
        id: 469u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord470 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_470() -> u64 {
    let record = ProofRecord470 {
        id: 470u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord471 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_471() -> u64 {
    let record = ProofRecord471 {
        id: 471u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord472 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_472() -> u64 {
    let record = ProofRecord472 {
        id: 472u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord473 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_473() -> u64 {
    let record = ProofRecord473 {
        id: 473u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord474 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_474() -> u64 {
    let record = ProofRecord474 {
        id: 474u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord475 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_475() -> u64 {
    let record = ProofRecord475 {
        id: 475u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord476 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_476() -> u64 {
    let record = ProofRecord476 {
        id: 476u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord477 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_477() -> u64 {
    let record = ProofRecord477 {
        id: 477u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord478 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_478() -> u64 {
    let record = ProofRecord478 {
        id: 478u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord479 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_479() -> u64 {
    let record = ProofRecord479 {
        id: 479u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord480 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_480() -> u64 {
    let record = ProofRecord480 {
        id: 480u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord481 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_481() -> u64 {
    let record = ProofRecord481 {
        id: 481u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord482 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_482() -> u64 {
    let record = ProofRecord482 {
        id: 482u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord483 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_483() -> u64 {
    let record = ProofRecord483 {
        id: 483u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord484 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_484() -> u64 {
    let record = ProofRecord484 {
        id: 484u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord485 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_485() -> u64 {
    let record = ProofRecord485 {
        id: 485u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord486 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_486() -> u64 {
    let record = ProofRecord486 {
        id: 486u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord487 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_487() -> u64 {
    let record = ProofRecord487 {
        id: 487u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord488 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_488() -> u64 {
    let record = ProofRecord488 {
        id: 488u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord489 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_489() -> u64 {
    let record = ProofRecord489 {
        id: 489u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord490 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_490() -> u64 {
    let record = ProofRecord490 {
        id: 490u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord491 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_491() -> u64 {
    let record = ProofRecord491 {
        id: 491u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord492 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_492() -> u64 {
    let record = ProofRecord492 {
        id: 492u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord493 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_493() -> u64 {
    let record = ProofRecord493 {
        id: 493u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord494 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_494() -> u64 {
    let record = ProofRecord494 {
        id: 494u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord495 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_495() -> u64 {
    let record = ProofRecord495 {
        id: 495u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord496 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_496() -> u64 {
    let record = ProofRecord496 {
        id: 496u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord497 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_497() -> u64 {
    let record = ProofRecord497 {
        id: 497u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord498 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_498() -> u64 {
    let record = ProofRecord498 {
        id: 498u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord499 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_499() -> u64 {
    let record = ProofRecord499 {
        id: 499u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord500 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_500() -> u64 {
    let record = ProofRecord500 {
        id: 500u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord501 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_501() -> u64 {
    let record = ProofRecord501 {
        id: 501u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord502 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_502() -> u64 {
    let record = ProofRecord502 {
        id: 502u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord503 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_503() -> u64 {
    let record = ProofRecord503 {
        id: 503u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord504 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_504() -> u64 {
    let record = ProofRecord504 {
        id: 504u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord505 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_505() -> u64 {
    let record = ProofRecord505 {
        id: 505u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord506 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_506() -> u64 {
    let record = ProofRecord506 {
        id: 506u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord507 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_507() -> u64 {
    let record = ProofRecord507 {
        id: 507u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord508 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_508() -> u64 {
    let record = ProofRecord508 {
        id: 508u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord509 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_509() -> u64 {
    let record = ProofRecord509 {
        id: 509u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord510 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_510() -> u64 {
    let record = ProofRecord510 {
        id: 510u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord511 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_511() -> u64 {
    let record = ProofRecord511 {
        id: 511u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord512 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_512() -> u64 {
    let record = ProofRecord512 {
        id: 512u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord513 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_513() -> u64 {
    let record = ProofRecord513 {
        id: 513u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord514 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_514() -> u64 {
    let record = ProofRecord514 {
        id: 514u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord515 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_515() -> u64 {
    let record = ProofRecord515 {
        id: 515u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord516 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_516() -> u64 {
    let record = ProofRecord516 {
        id: 516u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord517 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_517() -> u64 {
    let record = ProofRecord517 {
        id: 517u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord518 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_518() -> u64 {
    let record = ProofRecord518 {
        id: 518u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord519 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_519() -> u64 {
    let record = ProofRecord519 {
        id: 519u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord520 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_520() -> u64 {
    let record = ProofRecord520 {
        id: 520u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord521 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_521() -> u64 {
    let record = ProofRecord521 {
        id: 521u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord522 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_522() -> u64 {
    let record = ProofRecord522 {
        id: 522u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord523 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_523() -> u64 {
    let record = ProofRecord523 {
        id: 523u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord524 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_524() -> u64 {
    let record = ProofRecord524 {
        id: 524u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord525 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_525() -> u64 {
    let record = ProofRecord525 {
        id: 525u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord526 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_526() -> u64 {
    let record = ProofRecord526 {
        id: 526u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord527 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_527() -> u64 {
    let record = ProofRecord527 {
        id: 527u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord528 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_528() -> u64 {
    let record = ProofRecord528 {
        id: 528u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord529 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_529() -> u64 {
    let record = ProofRecord529 {
        id: 529u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord530 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_530() -> u64 {
    let record = ProofRecord530 {
        id: 530u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord531 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_531() -> u64 {
    let record = ProofRecord531 {
        id: 531u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord532 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_532() -> u64 {
    let record = ProofRecord532 {
        id: 532u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord533 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_533() -> u64 {
    let record = ProofRecord533 {
        id: 533u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord534 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_534() -> u64 {
    let record = ProofRecord534 {
        id: 534u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord535 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_535() -> u64 {
    let record = ProofRecord535 {
        id: 535u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord536 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_536() -> u64 {
    let record = ProofRecord536 {
        id: 536u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord537 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_537() -> u64 {
    let record = ProofRecord537 {
        id: 537u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord538 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_538() -> u64 {
    let record = ProofRecord538 {
        id: 538u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord539 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_539() -> u64 {
    let record = ProofRecord539 {
        id: 539u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord540 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_540() -> u64 {
    let record = ProofRecord540 {
        id: 540u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord541 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_541() -> u64 {
    let record = ProofRecord541 {
        id: 541u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord542 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_542() -> u64 {
    let record = ProofRecord542 {
        id: 542u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord543 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_543() -> u64 {
    let record = ProofRecord543 {
        id: 543u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord544 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_544() -> u64 {
    let record = ProofRecord544 {
        id: 544u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord545 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_545() -> u64 {
    let record = ProofRecord545 {
        id: 545u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord546 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_546() -> u64 {
    let record = ProofRecord546 {
        id: 546u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord547 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_547() -> u64 {
    let record = ProofRecord547 {
        id: 547u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord548 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_548() -> u64 {
    let record = ProofRecord548 {
        id: 548u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord549 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_549() -> u64 {
    let record = ProofRecord549 {
        id: 549u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord550 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_550() -> u64 {
    let record = ProofRecord550 {
        id: 550u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord551 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_551() -> u64 {
    let record = ProofRecord551 {
        id: 551u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord552 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_552() -> u64 {
    let record = ProofRecord552 {
        id: 552u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord553 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_553() -> u64 {
    let record = ProofRecord553 {
        id: 553u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord554 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_554() -> u64 {
    let record = ProofRecord554 {
        id: 554u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord555 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_555() -> u64 {
    let record = ProofRecord555 {
        id: 555u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord556 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_556() -> u64 {
    let record = ProofRecord556 {
        id: 556u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord557 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_557() -> u64 {
    let record = ProofRecord557 {
        id: 557u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord558 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_558() -> u64 {
    let record = ProofRecord558 {
        id: 558u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord559 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_559() -> u64 {
    let record = ProofRecord559 {
        id: 559u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord560 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_560() -> u64 {
    let record = ProofRecord560 {
        id: 560u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord561 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_561() -> u64 {
    let record = ProofRecord561 {
        id: 561u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord562 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_562() -> u64 {
    let record = ProofRecord562 {
        id: 562u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord563 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_563() -> u64 {
    let record = ProofRecord563 {
        id: 563u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord564 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_564() -> u64 {
    let record = ProofRecord564 {
        id: 564u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord565 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_565() -> u64 {
    let record = ProofRecord565 {
        id: 565u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord566 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_566() -> u64 {
    let record = ProofRecord566 {
        id: 566u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord567 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_567() -> u64 {
    let record = ProofRecord567 {
        id: 567u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord568 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_568() -> u64 {
    let record = ProofRecord568 {
        id: 568u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord569 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_569() -> u64 {
    let record = ProofRecord569 {
        id: 569u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord570 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_570() -> u64 {
    let record = ProofRecord570 {
        id: 570u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord571 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_571() -> u64 {
    let record = ProofRecord571 {
        id: 571u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord572 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_572() -> u64 {
    let record = ProofRecord572 {
        id: 572u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord573 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_573() -> u64 {
    let record = ProofRecord573 {
        id: 573u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord574 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_574() -> u64 {
    let record = ProofRecord574 {
        id: 574u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord575 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_575() -> u64 {
    let record = ProofRecord575 {
        id: 575u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord576 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_576() -> u64 {
    let record = ProofRecord576 {
        id: 576u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord577 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_577() -> u64 {
    let record = ProofRecord577 {
        id: 577u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord578 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_578() -> u64 {
    let record = ProofRecord578 {
        id: 578u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord579 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_579() -> u64 {
    let record = ProofRecord579 {
        id: 579u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord580 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_580() -> u64 {
    let record = ProofRecord580 {
        id: 580u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord581 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_581() -> u64 {
    let record = ProofRecord581 {
        id: 581u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord582 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_582() -> u64 {
    let record = ProofRecord582 {
        id: 582u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord583 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_583() -> u64 {
    let record = ProofRecord583 {
        id: 583u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord584 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_584() -> u64 {
    let record = ProofRecord584 {
        id: 584u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord585 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_585() -> u64 {
    let record = ProofRecord585 {
        id: 585u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord586 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_586() -> u64 {
    let record = ProofRecord586 {
        id: 586u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord587 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_587() -> u64 {
    let record = ProofRecord587 {
        id: 587u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord588 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_588() -> u64 {
    let record = ProofRecord588 {
        id: 588u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord589 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_589() -> u64 {
    let record = ProofRecord589 {
        id: 589u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord590 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_590() -> u64 {
    let record = ProofRecord590 {
        id: 590u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord591 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_591() -> u64 {
    let record = ProofRecord591 {
        id: 591u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord592 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_592() -> u64 {
    let record = ProofRecord592 {
        id: 592u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord593 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_593() -> u64 {
    let record = ProofRecord593 {
        id: 593u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord594 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_594() -> u64 {
    let record = ProofRecord594 {
        id: 594u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord595 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_595() -> u64 {
    let record = ProofRecord595 {
        id: 595u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord596 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_596() -> u64 {
    let record = ProofRecord596 {
        id: 596u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord597 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_597() -> u64 {
    let record = ProofRecord597 {
        id: 597u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord598 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_598() -> u64 {
    let record = ProofRecord598 {
        id: 598u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord599 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_599() -> u64 {
    let record = ProofRecord599 {
        id: 599u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord600 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_600() -> u64 {
    let record = ProofRecord600 {
        id: 600u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord601 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_601() -> u64 {
    let record = ProofRecord601 {
        id: 601u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord602 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_602() -> u64 {
    let record = ProofRecord602 {
        id: 602u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord603 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_603() -> u64 {
    let record = ProofRecord603 {
        id: 603u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord604 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_604() -> u64 {
    let record = ProofRecord604 {
        id: 604u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord605 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_605() -> u64 {
    let record = ProofRecord605 {
        id: 605u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord606 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_606() -> u64 {
    let record = ProofRecord606 {
        id: 606u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord607 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_607() -> u64 {
    let record = ProofRecord607 {
        id: 607u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord608 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_608() -> u64 {
    let record = ProofRecord608 {
        id: 608u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord609 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_609() -> u64 {
    let record = ProofRecord609 {
        id: 609u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord610 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_610() -> u64 {
    let record = ProofRecord610 {
        id: 610u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord611 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_611() -> u64 {
    let record = ProofRecord611 {
        id: 611u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord612 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_612() -> u64 {
    let record = ProofRecord612 {
        id: 612u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord613 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_613() -> u64 {
    let record = ProofRecord613 {
        id: 613u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord614 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_614() -> u64 {
    let record = ProofRecord614 {
        id: 614u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord615 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_615() -> u64 {
    let record = ProofRecord615 {
        id: 615u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord616 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_616() -> u64 {
    let record = ProofRecord616 {
        id: 616u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord617 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_617() -> u64 {
    let record = ProofRecord617 {
        id: 617u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord618 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_618() -> u64 {
    let record = ProofRecord618 {
        id: 618u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord619 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_619() -> u64 {
    let record = ProofRecord619 {
        id: 619u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord620 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_620() -> u64 {
    let record = ProofRecord620 {
        id: 620u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord621 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_621() -> u64 {
    let record = ProofRecord621 {
        id: 621u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord622 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_622() -> u64 {
    let record = ProofRecord622 {
        id: 622u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord623 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_623() -> u64 {
    let record = ProofRecord623 {
        id: 623u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord624 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_624() -> u64 {
    let record = ProofRecord624 {
        id: 624u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord625 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_625() -> u64 {
    let record = ProofRecord625 {
        id: 625u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord626 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_626() -> u64 {
    let record = ProofRecord626 {
        id: 626u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord627 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_627() -> u64 {
    let record = ProofRecord627 {
        id: 627u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord628 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_628() -> u64 {
    let record = ProofRecord628 {
        id: 628u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord629 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_629() -> u64 {
    let record = ProofRecord629 {
        id: 629u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord630 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_630() -> u64 {
    let record = ProofRecord630 {
        id: 630u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord631 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_631() -> u64 {
    let record = ProofRecord631 {
        id: 631u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord632 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_632() -> u64 {
    let record = ProofRecord632 {
        id: 632u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord633 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_633() -> u64 {
    let record = ProofRecord633 {
        id: 633u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord634 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_634() -> u64 {
    let record = ProofRecord634 {
        id: 634u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord635 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_635() -> u64 {
    let record = ProofRecord635 {
        id: 635u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord636 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_636() -> u64 {
    let record = ProofRecord636 {
        id: 636u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord637 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_637() -> u64 {
    let record = ProofRecord637 {
        id: 637u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord638 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_638() -> u64 {
    let record = ProofRecord638 {
        id: 638u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord639 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_639() -> u64 {
    let record = ProofRecord639 {
        id: 639u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord640 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_640() -> u64 {
    let record = ProofRecord640 {
        id: 640u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord641 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_641() -> u64 {
    let record = ProofRecord641 {
        id: 641u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord642 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_642() -> u64 {
    let record = ProofRecord642 {
        id: 642u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord643 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_643() -> u64 {
    let record = ProofRecord643 {
        id: 643u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord644 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_644() -> u64 {
    let record = ProofRecord644 {
        id: 644u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord645 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_645() -> u64 {
    let record = ProofRecord645 {
        id: 645u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord646 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_646() -> u64 {
    let record = ProofRecord646 {
        id: 646u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord647 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_647() -> u64 {
    let record = ProofRecord647 {
        id: 647u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord648 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_648() -> u64 {
    let record = ProofRecord648 {
        id: 648u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord649 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_649() -> u64 {
    let record = ProofRecord649 {
        id: 649u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord650 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_650() -> u64 {
    let record = ProofRecord650 {
        id: 650u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord651 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_651() -> u64 {
    let record = ProofRecord651 {
        id: 651u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord652 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_652() -> u64 {
    let record = ProofRecord652 {
        id: 652u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord653 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_653() -> u64 {
    let record = ProofRecord653 {
        id: 653u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord654 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_654() -> u64 {
    let record = ProofRecord654 {
        id: 654u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord655 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_655() -> u64 {
    let record = ProofRecord655 {
        id: 655u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord656 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_656() -> u64 {
    let record = ProofRecord656 {
        id: 656u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord657 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_657() -> u64 {
    let record = ProofRecord657 {
        id: 657u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord658 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_658() -> u64 {
    let record = ProofRecord658 {
        id: 658u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord659 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_659() -> u64 {
    let record = ProofRecord659 {
        id: 659u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord660 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_660() -> u64 {
    let record = ProofRecord660 {
        id: 660u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord661 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_661() -> u64 {
    let record = ProofRecord661 {
        id: 661u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord662 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_662() -> u64 {
    let record = ProofRecord662 {
        id: 662u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord663 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_663() -> u64 {
    let record = ProofRecord663 {
        id: 663u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord664 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_664() -> u64 {
    let record = ProofRecord664 {
        id: 664u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord665 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_665() -> u64 {
    let record = ProofRecord665 {
        id: 665u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord666 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_666() -> u64 {
    let record = ProofRecord666 {
        id: 666u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord667 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_667() -> u64 {
    let record = ProofRecord667 {
        id: 667u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord668 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_668() -> u64 {
    let record = ProofRecord668 {
        id: 668u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord669 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_669() -> u64 {
    let record = ProofRecord669 {
        id: 669u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord670 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_670() -> u64 {
    let record = ProofRecord670 {
        id: 670u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord671 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_671() -> u64 {
    let record = ProofRecord671 {
        id: 671u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord672 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_672() -> u64 {
    let record = ProofRecord672 {
        id: 672u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord673 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_673() -> u64 {
    let record = ProofRecord673 {
        id: 673u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord674 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_674() -> u64 {
    let record = ProofRecord674 {
        id: 674u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord675 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_675() -> u64 {
    let record = ProofRecord675 {
        id: 675u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord676 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_676() -> u64 {
    let record = ProofRecord676 {
        id: 676u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord677 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_677() -> u64 {
    let record = ProofRecord677 {
        id: 677u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord678 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_678() -> u64 {
    let record = ProofRecord678 {
        id: 678u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord679 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_679() -> u64 {
    let record = ProofRecord679 {
        id: 679u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord680 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_680() -> u64 {
    let record = ProofRecord680 {
        id: 680u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord681 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_681() -> u64 {
    let record = ProofRecord681 {
        id: 681u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord682 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_682() -> u64 {
    let record = ProofRecord682 {
        id: 682u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord683 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_683() -> u64 {
    let record = ProofRecord683 {
        id: 683u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord684 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_684() -> u64 {
    let record = ProofRecord684 {
        id: 684u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord685 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_685() -> u64 {
    let record = ProofRecord685 {
        id: 685u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord686 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_686() -> u64 {
    let record = ProofRecord686 {
        id: 686u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord687 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_687() -> u64 {
    let record = ProofRecord687 {
        id: 687u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord688 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_688() -> u64 {
    let record = ProofRecord688 {
        id: 688u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord689 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_689() -> u64 {
    let record = ProofRecord689 {
        id: 689u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord690 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_690() -> u64 {
    let record = ProofRecord690 {
        id: 690u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord691 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_691() -> u64 {
    let record = ProofRecord691 {
        id: 691u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord692 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_692() -> u64 {
    let record = ProofRecord692 {
        id: 692u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord693 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_693() -> u64 {
    let record = ProofRecord693 {
        id: 693u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord694 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_694() -> u64 {
    let record = ProofRecord694 {
        id: 694u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord695 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_695() -> u64 {
    let record = ProofRecord695 {
        id: 695u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord696 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_696() -> u64 {
    let record = ProofRecord696 {
        id: 696u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord697 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_697() -> u64 {
    let record = ProofRecord697 {
        id: 697u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord698 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_698() -> u64 {
    let record = ProofRecord698 {
        id: 698u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord699 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_699() -> u64 {
    let record = ProofRecord699 {
        id: 699u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord700 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_700() -> u64 {
    let record = ProofRecord700 {
        id: 700u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord701 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_701() -> u64 {
    let record = ProofRecord701 {
        id: 701u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord702 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_702() -> u64 {
    let record = ProofRecord702 {
        id: 702u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord703 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_703() -> u64 {
    let record = ProofRecord703 {
        id: 703u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord704 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_704() -> u64 {
    let record = ProofRecord704 {
        id: 704u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord705 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_705() -> u64 {
    let record = ProofRecord705 {
        id: 705u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord706 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_706() -> u64 {
    let record = ProofRecord706 {
        id: 706u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord707 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_707() -> u64 {
    let record = ProofRecord707 {
        id: 707u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord708 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_708() -> u64 {
    let record = ProofRecord708 {
        id: 708u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord709 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_709() -> u64 {
    let record = ProofRecord709 {
        id: 709u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord710 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_710() -> u64 {
    let record = ProofRecord710 {
        id: 710u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord711 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_711() -> u64 {
    let record = ProofRecord711 {
        id: 711u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord712 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_712() -> u64 {
    let record = ProofRecord712 {
        id: 712u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord713 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_713() -> u64 {
    let record = ProofRecord713 {
        id: 713u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord714 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_714() -> u64 {
    let record = ProofRecord714 {
        id: 714u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord715 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_715() -> u64 {
    let record = ProofRecord715 {
        id: 715u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord716 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_716() -> u64 {
    let record = ProofRecord716 {
        id: 716u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord717 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_717() -> u64 {
    let record = ProofRecord717 {
        id: 717u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord718 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_718() -> u64 {
    let record = ProofRecord718 {
        id: 718u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord719 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_719() -> u64 {
    let record = ProofRecord719 {
        id: 719u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord720 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_720() -> u64 {
    let record = ProofRecord720 {
        id: 720u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord721 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_721() -> u64 {
    let record = ProofRecord721 {
        id: 721u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord722 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_722() -> u64 {
    let record = ProofRecord722 {
        id: 722u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord723 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_723() -> u64 {
    let record = ProofRecord723 {
        id: 723u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord724 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_724() -> u64 {
    let record = ProofRecord724 {
        id: 724u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord725 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_725() -> u64 {
    let record = ProofRecord725 {
        id: 725u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord726 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_726() -> u64 {
    let record = ProofRecord726 {
        id: 726u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord727 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_727() -> u64 {
    let record = ProofRecord727 {
        id: 727u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord728 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_728() -> u64 {
    let record = ProofRecord728 {
        id: 728u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord729 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_729() -> u64 {
    let record = ProofRecord729 {
        id: 729u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord730 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_730() -> u64 {
    let record = ProofRecord730 {
        id: 730u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord731 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_731() -> u64 {
    let record = ProofRecord731 {
        id: 731u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord732 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_732() -> u64 {
    let record = ProofRecord732 {
        id: 732u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord733 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_733() -> u64 {
    let record = ProofRecord733 {
        id: 733u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord734 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_734() -> u64 {
    let record = ProofRecord734 {
        id: 734u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord735 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_735() -> u64 {
    let record = ProofRecord735 {
        id: 735u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord736 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_736() -> u64 {
    let record = ProofRecord736 {
        id: 736u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord737 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_737() -> u64 {
    let record = ProofRecord737 {
        id: 737u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord738 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_738() -> u64 {
    let record = ProofRecord738 {
        id: 738u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord739 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_739() -> u64 {
    let record = ProofRecord739 {
        id: 739u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord740 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_740() -> u64 {
    let record = ProofRecord740 {
        id: 740u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord741 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_741() -> u64 {
    let record = ProofRecord741 {
        id: 741u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord742 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_742() -> u64 {
    let record = ProofRecord742 {
        id: 742u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord743 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_743() -> u64 {
    let record = ProofRecord743 {
        id: 743u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord744 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_744() -> u64 {
    let record = ProofRecord744 {
        id: 744u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord745 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_745() -> u64 {
    let record = ProofRecord745 {
        id: 745u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord746 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_746() -> u64 {
    let record = ProofRecord746 {
        id: 746u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord747 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_747() -> u64 {
    let record = ProofRecord747 {
        id: 747u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord748 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_748() -> u64 {
    let record = ProofRecord748 {
        id: 748u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord749 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_749() -> u64 {
    let record = ProofRecord749 {
        id: 749u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord750 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_750() -> u64 {
    let record = ProofRecord750 {
        id: 750u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord751 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_751() -> u64 {
    let record = ProofRecord751 {
        id: 751u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord752 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_752() -> u64 {
    let record = ProofRecord752 {
        id: 752u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord753 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_753() -> u64 {
    let record = ProofRecord753 {
        id: 753u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord754 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_754() -> u64 {
    let record = ProofRecord754 {
        id: 754u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord755 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_755() -> u64 {
    let record = ProofRecord755 {
        id: 755u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord756 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_756() -> u64 {
    let record = ProofRecord756 {
        id: 756u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord757 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_757() -> u64 {
    let record = ProofRecord757 {
        id: 757u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord758 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_758() -> u64 {
    let record = ProofRecord758 {
        id: 758u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord759 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_759() -> u64 {
    let record = ProofRecord759 {
        id: 759u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord760 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_760() -> u64 {
    let record = ProofRecord760 {
        id: 760u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord761 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_761() -> u64 {
    let record = ProofRecord761 {
        id: 761u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord762 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_762() -> u64 {
    let record = ProofRecord762 {
        id: 762u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord763 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_763() -> u64 {
    let record = ProofRecord763 {
        id: 763u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord764 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_764() -> u64 {
    let record = ProofRecord764 {
        id: 764u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord765 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_765() -> u64 {
    let record = ProofRecord765 {
        id: 765u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord766 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_766() -> u64 {
    let record = ProofRecord766 {
        id: 766u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord767 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_767() -> u64 {
    let record = ProofRecord767 {
        id: 767u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord768 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_768() -> u64 {
    let record = ProofRecord768 {
        id: 768u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord769 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_769() -> u64 {
    let record = ProofRecord769 {
        id: 769u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord770 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_770() -> u64 {
    let record = ProofRecord770 {
        id: 770u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord771 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_771() -> u64 {
    let record = ProofRecord771 {
        id: 771u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord772 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_772() -> u64 {
    let record = ProofRecord772 {
        id: 772u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord773 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_773() -> u64 {
    let record = ProofRecord773 {
        id: 773u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord774 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_774() -> u64 {
    let record = ProofRecord774 {
        id: 774u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord775 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_775() -> u64 {
    let record = ProofRecord775 {
        id: 775u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord776 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_776() -> u64 {
    let record = ProofRecord776 {
        id: 776u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord777 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_777() -> u64 {
    let record = ProofRecord777 {
        id: 777u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord778 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_778() -> u64 {
    let record = ProofRecord778 {
        id: 778u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord779 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_779() -> u64 {
    let record = ProofRecord779 {
        id: 779u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord780 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_780() -> u64 {
    let record = ProofRecord780 {
        id: 780u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord781 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_781() -> u64 {
    let record = ProofRecord781 {
        id: 781u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord782 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_782() -> u64 {
    let record = ProofRecord782 {
        id: 782u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord783 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_783() -> u64 {
    let record = ProofRecord783 {
        id: 783u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord784 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_784() -> u64 {
    let record = ProofRecord784 {
        id: 784u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord785 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_785() -> u64 {
    let record = ProofRecord785 {
        id: 785u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord786 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_786() -> u64 {
    let record = ProofRecord786 {
        id: 786u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord787 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_787() -> u64 {
    let record = ProofRecord787 {
        id: 787u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord788 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_788() -> u64 {
    let record = ProofRecord788 {
        id: 788u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord789 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_789() -> u64 {
    let record = ProofRecord789 {
        id: 789u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord790 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_790() -> u64 {
    let record = ProofRecord790 {
        id: 790u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord791 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_791() -> u64 {
    let record = ProofRecord791 {
        id: 791u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord792 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_792() -> u64 {
    let record = ProofRecord792 {
        id: 792u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord793 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_793() -> u64 {
    let record = ProofRecord793 {
        id: 793u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord794 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_794() -> u64 {
    let record = ProofRecord794 {
        id: 794u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord795 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_795() -> u64 {
    let record = ProofRecord795 {
        id: 795u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord796 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_796() -> u64 {
    let record = ProofRecord796 {
        id: 796u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord797 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_797() -> u64 {
    let record = ProofRecord797 {
        id: 797u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord798 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_798() -> u64 {
    let record = ProofRecord798 {
        id: 798u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord799 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_799() -> u64 {
    let record = ProofRecord799 {
        id: 799u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord800 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_800() -> u64 {
    let record = ProofRecord800 {
        id: 800u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord801 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_801() -> u64 {
    let record = ProofRecord801 {
        id: 801u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord802 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_802() -> u64 {
    let record = ProofRecord802 {
        id: 802u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord803 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_803() -> u64 {
    let record = ProofRecord803 {
        id: 803u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord804 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_804() -> u64 {
    let record = ProofRecord804 {
        id: 804u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord805 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_805() -> u64 {
    let record = ProofRecord805 {
        id: 805u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord806 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_806() -> u64 {
    let record = ProofRecord806 {
        id: 806u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord807 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_807() -> u64 {
    let record = ProofRecord807 {
        id: 807u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord808 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_808() -> u64 {
    let record = ProofRecord808 {
        id: 808u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord809 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_809() -> u64 {
    let record = ProofRecord809 {
        id: 809u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord810 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_810() -> u64 {
    let record = ProofRecord810 {
        id: 810u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord811 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_811() -> u64 {
    let record = ProofRecord811 {
        id: 811u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord812 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_812() -> u64 {
    let record = ProofRecord812 {
        id: 812u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord813 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_813() -> u64 {
    let record = ProofRecord813 {
        id: 813u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord814 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_814() -> u64 {
    let record = ProofRecord814 {
        id: 814u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord815 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_815() -> u64 {
    let record = ProofRecord815 {
        id: 815u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord816 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_816() -> u64 {
    let record = ProofRecord816 {
        id: 816u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord817 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_817() -> u64 {
    let record = ProofRecord817 {
        id: 817u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord818 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_818() -> u64 {
    let record = ProofRecord818 {
        id: 818u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord819 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_819() -> u64 {
    let record = ProofRecord819 {
        id: 819u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord820 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_820() -> u64 {
    let record = ProofRecord820 {
        id: 820u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord821 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_821() -> u64 {
    let record = ProofRecord821 {
        id: 821u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord822 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_822() -> u64 {
    let record = ProofRecord822 {
        id: 822u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord823 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_823() -> u64 {
    let record = ProofRecord823 {
        id: 823u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord824 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_824() -> u64 {
    let record = ProofRecord824 {
        id: 824u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord825 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_825() -> u64 {
    let record = ProofRecord825 {
        id: 825u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord826 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_826() -> u64 {
    let record = ProofRecord826 {
        id: 826u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord827 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_827() -> u64 {
    let record = ProofRecord827 {
        id: 827u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord828 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_828() -> u64 {
    let record = ProofRecord828 {
        id: 828u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord829 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_829() -> u64 {
    let record = ProofRecord829 {
        id: 829u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord830 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_830() -> u64 {
    let record = ProofRecord830 {
        id: 830u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord831 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_831() -> u64 {
    let record = ProofRecord831 {
        id: 831u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord832 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_832() -> u64 {
    let record = ProofRecord832 {
        id: 832u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord833 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_833() -> u64 {
    let record = ProofRecord833 {
        id: 833u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord834 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_834() -> u64 {
    let record = ProofRecord834 {
        id: 834u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord835 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_835() -> u64 {
    let record = ProofRecord835 {
        id: 835u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord836 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_836() -> u64 {
    let record = ProofRecord836 {
        id: 836u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord837 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_837() -> u64 {
    let record = ProofRecord837 {
        id: 837u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord838 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_838() -> u64 {
    let record = ProofRecord838 {
        id: 838u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord839 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_839() -> u64 {
    let record = ProofRecord839 {
        id: 839u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord840 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_840() -> u64 {
    let record = ProofRecord840 {
        id: 840u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord841 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_841() -> u64 {
    let record = ProofRecord841 {
        id: 841u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord842 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_842() -> u64 {
    let record = ProofRecord842 {
        id: 842u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord843 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_843() -> u64 {
    let record = ProofRecord843 {
        id: 843u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord844 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_844() -> u64 {
    let record = ProofRecord844 {
        id: 844u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord845 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_845() -> u64 {
    let record = ProofRecord845 {
        id: 845u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord846 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_846() -> u64 {
    let record = ProofRecord846 {
        id: 846u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord847 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_847() -> u64 {
    let record = ProofRecord847 {
        id: 847u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord848 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_848() -> u64 {
    let record = ProofRecord848 {
        id: 848u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord849 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_849() -> u64 {
    let record = ProofRecord849 {
        id: 849u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord850 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_850() -> u64 {
    let record = ProofRecord850 {
        id: 850u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord851 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_851() -> u64 {
    let record = ProofRecord851 {
        id: 851u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord852 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_852() -> u64 {
    let record = ProofRecord852 {
        id: 852u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord853 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_853() -> u64 {
    let record = ProofRecord853 {
        id: 853u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord854 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_854() -> u64 {
    let record = ProofRecord854 {
        id: 854u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord855 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_855() -> u64 {
    let record = ProofRecord855 {
        id: 855u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord856 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_856() -> u64 {
    let record = ProofRecord856 {
        id: 856u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord857 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_857() -> u64 {
    let record = ProofRecord857 {
        id: 857u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord858 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_858() -> u64 {
    let record = ProofRecord858 {
        id: 858u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord859 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_859() -> u64 {
    let record = ProofRecord859 {
        id: 859u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord860 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_860() -> u64 {
    let record = ProofRecord860 {
        id: 860u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord861 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_861() -> u64 {
    let record = ProofRecord861 {
        id: 861u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord862 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_862() -> u64 {
    let record = ProofRecord862 {
        id: 862u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord863 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_863() -> u64 {
    let record = ProofRecord863 {
        id: 863u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord864 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_864() -> u64 {
    let record = ProofRecord864 {
        id: 864u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord865 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_865() -> u64 {
    let record = ProofRecord865 {
        id: 865u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord866 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_866() -> u64 {
    let record = ProofRecord866 {
        id: 866u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord867 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_867() -> u64 {
    let record = ProofRecord867 {
        id: 867u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord868 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_868() -> u64 {
    let record = ProofRecord868 {
        id: 868u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord869 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_869() -> u64 {
    let record = ProofRecord869 {
        id: 869u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord870 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_870() -> u64 {
    let record = ProofRecord870 {
        id: 870u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord871 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_871() -> u64 {
    let record = ProofRecord871 {
        id: 871u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord872 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_872() -> u64 {
    let record = ProofRecord872 {
        id: 872u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord873 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_873() -> u64 {
    let record = ProofRecord873 {
        id: 873u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord874 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_874() -> u64 {
    let record = ProofRecord874 {
        id: 874u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord875 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_875() -> u64 {
    let record = ProofRecord875 {
        id: 875u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord876 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_876() -> u64 {
    let record = ProofRecord876 {
        id: 876u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord877 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_877() -> u64 {
    let record = ProofRecord877 {
        id: 877u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord878 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_878() -> u64 {
    let record = ProofRecord878 {
        id: 878u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord879 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_879() -> u64 {
    let record = ProofRecord879 {
        id: 879u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord880 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_880() -> u64 {
    let record = ProofRecord880 {
        id: 880u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord881 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_881() -> u64 {
    let record = ProofRecord881 {
        id: 881u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord882 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_882() -> u64 {
    let record = ProofRecord882 {
        id: 882u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord883 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_883() -> u64 {
    let record = ProofRecord883 {
        id: 883u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord884 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_884() -> u64 {
    let record = ProofRecord884 {
        id: 884u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord885 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_885() -> u64 {
    let record = ProofRecord885 {
        id: 885u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord886 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_886() -> u64 {
    let record = ProofRecord886 {
        id: 886u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord887 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_887() -> u64 {
    let record = ProofRecord887 {
        id: 887u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord888 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_888() -> u64 {
    let record = ProofRecord888 {
        id: 888u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord889 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_889() -> u64 {
    let record = ProofRecord889 {
        id: 889u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord890 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_890() -> u64 {
    let record = ProofRecord890 {
        id: 890u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord891 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_891() -> u64 {
    let record = ProofRecord891 {
        id: 891u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord892 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_892() -> u64 {
    let record = ProofRecord892 {
        id: 892u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord893 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_893() -> u64 {
    let record = ProofRecord893 {
        id: 893u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord894 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_894() -> u64 {
    let record = ProofRecord894 {
        id: 894u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord895 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_895() -> u64 {
    let record = ProofRecord895 {
        id: 895u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord896 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_896() -> u64 {
    let record = ProofRecord896 {
        id: 896u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord897 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_897() -> u64 {
    let record = ProofRecord897 {
        id: 897u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord898 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_898() -> u64 {
    let record = ProofRecord898 {
        id: 898u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord899 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_899() -> u64 {
    let record = ProofRecord899 {
        id: 899u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord900 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_900() -> u64 {
    let record = ProofRecord900 {
        id: 900u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord901 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_901() -> u64 {
    let record = ProofRecord901 {
        id: 901u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord902 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_902() -> u64 {
    let record = ProofRecord902 {
        id: 902u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord903 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_903() -> u64 {
    let record = ProofRecord903 {
        id: 903u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord904 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_904() -> u64 {
    let record = ProofRecord904 {
        id: 904u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord905 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_905() -> u64 {
    let record = ProofRecord905 {
        id: 905u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord906 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_906() -> u64 {
    let record = ProofRecord906 {
        id: 906u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord907 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_907() -> u64 {
    let record = ProofRecord907 {
        id: 907u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord908 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_908() -> u64 {
    let record = ProofRecord908 {
        id: 908u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord909 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_909() -> u64 {
    let record = ProofRecord909 {
        id: 909u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord910 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_910() -> u64 {
    let record = ProofRecord910 {
        id: 910u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord911 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_911() -> u64 {
    let record = ProofRecord911 {
        id: 911u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord912 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_912() -> u64 {
    let record = ProofRecord912 {
        id: 912u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord913 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_913() -> u64 {
    let record = ProofRecord913 {
        id: 913u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord914 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_914() -> u64 {
    let record = ProofRecord914 {
        id: 914u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord915 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_915() -> u64 {
    let record = ProofRecord915 {
        id: 915u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord916 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_916() -> u64 {
    let record = ProofRecord916 {
        id: 916u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord917 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_917() -> u64 {
    let record = ProofRecord917 {
        id: 917u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord918 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_918() -> u64 {
    let record = ProofRecord918 {
        id: 918u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord919 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_919() -> u64 {
    let record = ProofRecord919 {
        id: 919u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord920 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_920() -> u64 {
    let record = ProofRecord920 {
        id: 920u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord921 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_921() -> u64 {
    let record = ProofRecord921 {
        id: 921u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord922 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_922() -> u64 {
    let record = ProofRecord922 {
        id: 922u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord923 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_923() -> u64 {
    let record = ProofRecord923 {
        id: 923u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord924 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_924() -> u64 {
    let record = ProofRecord924 {
        id: 924u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord925 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_925() -> u64 {
    let record = ProofRecord925 {
        id: 925u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord926 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_926() -> u64 {
    let record = ProofRecord926 {
        id: 926u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord927 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_927() -> u64 {
    let record = ProofRecord927 {
        id: 927u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord928 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_928() -> u64 {
    let record = ProofRecord928 {
        id: 928u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord929 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_929() -> u64 {
    let record = ProofRecord929 {
        id: 929u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord930 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_930() -> u64 {
    let record = ProofRecord930 {
        id: 930u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord931 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_931() -> u64 {
    let record = ProofRecord931 {
        id: 931u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord932 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_932() -> u64 {
    let record = ProofRecord932 {
        id: 932u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord933 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_933() -> u64 {
    let record = ProofRecord933 {
        id: 933u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord934 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_934() -> u64 {
    let record = ProofRecord934 {
        id: 934u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord935 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_935() -> u64 {
    let record = ProofRecord935 {
        id: 935u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord936 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_936() -> u64 {
    let record = ProofRecord936 {
        id: 936u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord937 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_937() -> u64 {
    let record = ProofRecord937 {
        id: 937u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord938 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_938() -> u64 {
    let record = ProofRecord938 {
        id: 938u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord939 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_939() -> u64 {
    let record = ProofRecord939 {
        id: 939u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord940 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_940() -> u64 {
    let record = ProofRecord940 {
        id: 940u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord941 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_941() -> u64 {
    let record = ProofRecord941 {
        id: 941u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord942 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_942() -> u64 {
    let record = ProofRecord942 {
        id: 942u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord943 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_943() -> u64 {
    let record = ProofRecord943 {
        id: 943u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord944 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_944() -> u64 {
    let record = ProofRecord944 {
        id: 944u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord945 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_945() -> u64 {
    let record = ProofRecord945 {
        id: 945u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord946 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_946() -> u64 {
    let record = ProofRecord946 {
        id: 946u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord947 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_947() -> u64 {
    let record = ProofRecord947 {
        id: 947u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord948 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_948() -> u64 {
    let record = ProofRecord948 {
        id: 948u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord949 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_949() -> u64 {
    let record = ProofRecord949 {
        id: 949u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord950 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_950() -> u64 {
    let record = ProofRecord950 {
        id: 950u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord951 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_951() -> u64 {
    let record = ProofRecord951 {
        id: 951u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord952 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_952() -> u64 {
    let record = ProofRecord952 {
        id: 952u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord953 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_953() -> u64 {
    let record = ProofRecord953 {
        id: 953u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord954 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_954() -> u64 {
    let record = ProofRecord954 {
        id: 954u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord955 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_955() -> u64 {
    let record = ProofRecord955 {
        id: 955u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord956 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_956() -> u64 {
    let record = ProofRecord956 {
        id: 956u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord957 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_957() -> u64 {
    let record = ProofRecord957 {
        id: 957u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord958 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_958() -> u64 {
    let record = ProofRecord958 {
        id: 958u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord959 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_959() -> u64 {
    let record = ProofRecord959 {
        id: 959u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord960 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_960() -> u64 {
    let record = ProofRecord960 {
        id: 960u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord961 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_961() -> u64 {
    let record = ProofRecord961 {
        id: 961u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord962 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_962() -> u64 {
    let record = ProofRecord962 {
        id: 962u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord963 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_963() -> u64 {
    let record = ProofRecord963 {
        id: 963u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord964 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_964() -> u64 {
    let record = ProofRecord964 {
        id: 964u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord965 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_965() -> u64 {
    let record = ProofRecord965 {
        id: 965u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord966 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_966() -> u64 {
    let record = ProofRecord966 {
        id: 966u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord967 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_967() -> u64 {
    let record = ProofRecord967 {
        id: 967u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord968 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_968() -> u64 {
    let record = ProofRecord968 {
        id: 968u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord969 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_969() -> u64 {
    let record = ProofRecord969 {
        id: 969u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord970 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_970() -> u64 {
    let record = ProofRecord970 {
        id: 970u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord971 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_971() -> u64 {
    let record = ProofRecord971 {
        id: 971u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord972 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_972() -> u64 {
    let record = ProofRecord972 {
        id: 972u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord973 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_973() -> u64 {
    let record = ProofRecord973 {
        id: 973u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord974 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_974() -> u64 {
    let record = ProofRecord974 {
        id: 974u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord975 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_975() -> u64 {
    let record = ProofRecord975 {
        id: 975u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord976 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_976() -> u64 {
    let record = ProofRecord976 {
        id: 976u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord977 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_977() -> u64 {
    let record = ProofRecord977 {
        id: 977u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord978 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_978() -> u64 {
    let record = ProofRecord978 {
        id: 978u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord979 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_979() -> u64 {
    let record = ProofRecord979 {
        id: 979u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord980 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_980() -> u64 {
    let record = ProofRecord980 {
        id: 980u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord981 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_981() -> u64 {
    let record = ProofRecord981 {
        id: 981u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord982 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_982() -> u64 {
    let record = ProofRecord982 {
        id: 982u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord983 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_983() -> u64 {
    let record = ProofRecord983 {
        id: 983u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord984 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_984() -> u64 {
    let record = ProofRecord984 {
        id: 984u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord985 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_985() -> u64 {
    let record = ProofRecord985 {
        id: 985u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord986 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_986() -> u64 {
    let record = ProofRecord986 {
        id: 986u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord987 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_987() -> u64 {
    let record = ProofRecord987 {
        id: 987u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord988 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_988() -> u64 {
    let record = ProofRecord988 {
        id: 988u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord989 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_989() -> u64 {
    let record = ProofRecord989 {
        id: 989u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord990 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_990() -> u64 {
    let record = ProofRecord990 {
        id: 990u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord991 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_991() -> u64 {
    let record = ProofRecord991 {
        id: 991u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord992 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_992() -> u64 {
    let record = ProofRecord992 {
        id: 992u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord993 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_993() -> u64 {
    let record = ProofRecord993 {
        id: 993u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord994 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_994() -> u64 {
    let record = ProofRecord994 {
        id: 994u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord995 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_995() -> u64 {
    let record = ProofRecord995 {
        id: 995u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord996 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_996() -> u64 {
    let record = ProofRecord996 {
        id: 996u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord997 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_997() -> u64 {
    let record = ProofRecord997 {
        id: 997u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord998 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_998() -> u64 {
    let record = ProofRecord998 {
        id: 998u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord999 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_999() -> u64 {
    let record = ProofRecord999 {
        id: 999u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1000 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1000() -> u64 {
    let record = ProofRecord1000 {
        id: 1000u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1001 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1001() -> u64 {
    let record = ProofRecord1001 {
        id: 1001u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1002 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1002() -> u64 {
    let record = ProofRecord1002 {
        id: 1002u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1003 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1003() -> u64 {
    let record = ProofRecord1003 {
        id: 1003u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1004 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1004() -> u64 {
    let record = ProofRecord1004 {
        id: 1004u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1005 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1005() -> u64 {
    let record = ProofRecord1005 {
        id: 1005u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1006 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1006() -> u64 {
    let record = ProofRecord1006 {
        id: 1006u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1007 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1007() -> u64 {
    let record = ProofRecord1007 {
        id: 1007u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1008 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1008() -> u64 {
    let record = ProofRecord1008 {
        id: 1008u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1009 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1009() -> u64 {
    let record = ProofRecord1009 {
        id: 1009u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1010 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1010() -> u64 {
    let record = ProofRecord1010 {
        id: 1010u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1011 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1011() -> u64 {
    let record = ProofRecord1011 {
        id: 1011u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1012 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1012() -> u64 {
    let record = ProofRecord1012 {
        id: 1012u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1013 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1013() -> u64 {
    let record = ProofRecord1013 {
        id: 1013u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1014 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1014() -> u64 {
    let record = ProofRecord1014 {
        id: 1014u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1015 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1015() -> u64 {
    let record = ProofRecord1015 {
        id: 1015u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1016 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1016() -> u64 {
    let record = ProofRecord1016 {
        id: 1016u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1017 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1017() -> u64 {
    let record = ProofRecord1017 {
        id: 1017u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1018 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1018() -> u64 {
    let record = ProofRecord1018 {
        id: 1018u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1019 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1019() -> u64 {
    let record = ProofRecord1019 {
        id: 1019u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1020 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1020() -> u64 {
    let record = ProofRecord1020 {
        id: 1020u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1021 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1021() -> u64 {
    let record = ProofRecord1021 {
        id: 1021u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1022 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1022() -> u64 {
    let record = ProofRecord1022 {
        id: 1022u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1023 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1023() -> u64 {
    let record = ProofRecord1023 {
        id: 1023u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1024 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1024() -> u64 {
    let record = ProofRecord1024 {
        id: 1024u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1025 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1025() -> u64 {
    let record = ProofRecord1025 {
        id: 1025u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1026 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1026() -> u64 {
    let record = ProofRecord1026 {
        id: 1026u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1027 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1027() -> u64 {
    let record = ProofRecord1027 {
        id: 1027u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1028 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1028() -> u64 {
    let record = ProofRecord1028 {
        id: 1028u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1029 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1029() -> u64 {
    let record = ProofRecord1029 {
        id: 1029u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1030 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1030() -> u64 {
    let record = ProofRecord1030 {
        id: 1030u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1031 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1031() -> u64 {
    let record = ProofRecord1031 {
        id: 1031u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1032 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1032() -> u64 {
    let record = ProofRecord1032 {
        id: 1032u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1033 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1033() -> u64 {
    let record = ProofRecord1033 {
        id: 1033u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1034 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1034() -> u64 {
    let record = ProofRecord1034 {
        id: 1034u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1035 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1035() -> u64 {
    let record = ProofRecord1035 {
        id: 1035u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1036 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1036() -> u64 {
    let record = ProofRecord1036 {
        id: 1036u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1037 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1037() -> u64 {
    let record = ProofRecord1037 {
        id: 1037u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1038 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1038() -> u64 {
    let record = ProofRecord1038 {
        id: 1038u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1039 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1039() -> u64 {
    let record = ProofRecord1039 {
        id: 1039u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1040 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1040() -> u64 {
    let record = ProofRecord1040 {
        id: 1040u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1041 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1041() -> u64 {
    let record = ProofRecord1041 {
        id: 1041u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1042 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1042() -> u64 {
    let record = ProofRecord1042 {
        id: 1042u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1043 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1043() -> u64 {
    let record = ProofRecord1043 {
        id: 1043u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1044 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1044() -> u64 {
    let record = ProofRecord1044 {
        id: 1044u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1045 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1045() -> u64 {
    let record = ProofRecord1045 {
        id: 1045u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1046 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1046() -> u64 {
    let record = ProofRecord1046 {
        id: 1046u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1047 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1047() -> u64 {
    let record = ProofRecord1047 {
        id: 1047u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1048 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1048() -> u64 {
    let record = ProofRecord1048 {
        id: 1048u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1049 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1049() -> u64 {
    let record = ProofRecord1049 {
        id: 1049u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1050 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1050() -> u64 {
    let record = ProofRecord1050 {
        id: 1050u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1051 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1051() -> u64 {
    let record = ProofRecord1051 {
        id: 1051u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1052 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1052() -> u64 {
    let record = ProofRecord1052 {
        id: 1052u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1053 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1053() -> u64 {
    let record = ProofRecord1053 {
        id: 1053u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1054 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1054() -> u64 {
    let record = ProofRecord1054 {
        id: 1054u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1055 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1055() -> u64 {
    let record = ProofRecord1055 {
        id: 1055u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1056 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1056() -> u64 {
    let record = ProofRecord1056 {
        id: 1056u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1057 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1057() -> u64 {
    let record = ProofRecord1057 {
        id: 1057u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1058 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1058() -> u64 {
    let record = ProofRecord1058 {
        id: 1058u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1059 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1059() -> u64 {
    let record = ProofRecord1059 {
        id: 1059u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1060 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1060() -> u64 {
    let record = ProofRecord1060 {
        id: 1060u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1061 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1061() -> u64 {
    let record = ProofRecord1061 {
        id: 1061u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1062 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1062() -> u64 {
    let record = ProofRecord1062 {
        id: 1062u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1063 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1063() -> u64 {
    let record = ProofRecord1063 {
        id: 1063u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1064 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1064() -> u64 {
    let record = ProofRecord1064 {
        id: 1064u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1065 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1065() -> u64 {
    let record = ProofRecord1065 {
        id: 1065u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1066 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1066() -> u64 {
    let record = ProofRecord1066 {
        id: 1066u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1067 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1067() -> u64 {
    let record = ProofRecord1067 {
        id: 1067u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1068 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1068() -> u64 {
    let record = ProofRecord1068 {
        id: 1068u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1069 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1069() -> u64 {
    let record = ProofRecord1069 {
        id: 1069u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1070 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1070() -> u64 {
    let record = ProofRecord1070 {
        id: 1070u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1071 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1071() -> u64 {
    let record = ProofRecord1071 {
        id: 1071u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1072 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1072() -> u64 {
    let record = ProofRecord1072 {
        id: 1072u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1073 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1073() -> u64 {
    let record = ProofRecord1073 {
        id: 1073u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1074 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1074() -> u64 {
    let record = ProofRecord1074 {
        id: 1074u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1075 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1075() -> u64 {
    let record = ProofRecord1075 {
        id: 1075u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1076 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1076() -> u64 {
    let record = ProofRecord1076 {
        id: 1076u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1077 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1077() -> u64 {
    let record = ProofRecord1077 {
        id: 1077u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1078 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1078() -> u64 {
    let record = ProofRecord1078 {
        id: 1078u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1079 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1079() -> u64 {
    let record = ProofRecord1079 {
        id: 1079u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1080 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1080() -> u64 {
    let record = ProofRecord1080 {
        id: 1080u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1081 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1081() -> u64 {
    let record = ProofRecord1081 {
        id: 1081u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1082 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1082() -> u64 {
    let record = ProofRecord1082 {
        id: 1082u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1083 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1083() -> u64 {
    let record = ProofRecord1083 {
        id: 1083u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1084 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1084() -> u64 {
    let record = ProofRecord1084 {
        id: 1084u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1085 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1085() -> u64 {
    let record = ProofRecord1085 {
        id: 1085u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1086 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1086() -> u64 {
    let record = ProofRecord1086 {
        id: 1086u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1087 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1087() -> u64 {
    let record = ProofRecord1087 {
        id: 1087u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1088 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1088() -> u64 {
    let record = ProofRecord1088 {
        id: 1088u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1089 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1089() -> u64 {
    let record = ProofRecord1089 {
        id: 1089u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1090 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1090() -> u64 {
    let record = ProofRecord1090 {
        id: 1090u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1091 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1091() -> u64 {
    let record = ProofRecord1091 {
        id: 1091u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1092 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1092() -> u64 {
    let record = ProofRecord1092 {
        id: 1092u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1093 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1093() -> u64 {
    let record = ProofRecord1093 {
        id: 1093u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1094 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1094() -> u64 {
    let record = ProofRecord1094 {
        id: 1094u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1095 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1095() -> u64 {
    let record = ProofRecord1095 {
        id: 1095u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1096 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1096() -> u64 {
    let record = ProofRecord1096 {
        id: 1096u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1097 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1097() -> u64 {
    let record = ProofRecord1097 {
        id: 1097u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1098 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1098() -> u64 {
    let record = ProofRecord1098 {
        id: 1098u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1099 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1099() -> u64 {
    let record = ProofRecord1099 {
        id: 1099u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1100 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1100() -> u64 {
    let record = ProofRecord1100 {
        id: 1100u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1101 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1101() -> u64 {
    let record = ProofRecord1101 {
        id: 1101u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1102 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1102() -> u64 {
    let record = ProofRecord1102 {
        id: 1102u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1103 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1103() -> u64 {
    let record = ProofRecord1103 {
        id: 1103u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1104 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1104() -> u64 {
    let record = ProofRecord1104 {
        id: 1104u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1105 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1105() -> u64 {
    let record = ProofRecord1105 {
        id: 1105u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1106 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1106() -> u64 {
    let record = ProofRecord1106 {
        id: 1106u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1107 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1107() -> u64 {
    let record = ProofRecord1107 {
        id: 1107u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1108 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1108() -> u64 {
    let record = ProofRecord1108 {
        id: 1108u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1109 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1109() -> u64 {
    let record = ProofRecord1109 {
        id: 1109u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1110 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1110() -> u64 {
    let record = ProofRecord1110 {
        id: 1110u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1111 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1111() -> u64 {
    let record = ProofRecord1111 {
        id: 1111u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1112 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1112() -> u64 {
    let record = ProofRecord1112 {
        id: 1112u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1113 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1113() -> u64 {
    let record = ProofRecord1113 {
        id: 1113u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1114 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1114() -> u64 {
    let record = ProofRecord1114 {
        id: 1114u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1115 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1115() -> u64 {
    let record = ProofRecord1115 {
        id: 1115u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1116 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1116() -> u64 {
    let record = ProofRecord1116 {
        id: 1116u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1117 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1117() -> u64 {
    let record = ProofRecord1117 {
        id: 1117u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1118 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1118() -> u64 {
    let record = ProofRecord1118 {
        id: 1118u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1119 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1119() -> u64 {
    let record = ProofRecord1119 {
        id: 1119u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1120 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1120() -> u64 {
    let record = ProofRecord1120 {
        id: 1120u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1121 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1121() -> u64 {
    let record = ProofRecord1121 {
        id: 1121u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1122 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1122() -> u64 {
    let record = ProofRecord1122 {
        id: 1122u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1123 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1123() -> u64 {
    let record = ProofRecord1123 {
        id: 1123u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1124 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1124() -> u64 {
    let record = ProofRecord1124 {
        id: 1124u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1125 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1125() -> u64 {
    let record = ProofRecord1125 {
        id: 1125u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1126 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1126() -> u64 {
    let record = ProofRecord1126 {
        id: 1126u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1127 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1127() -> u64 {
    let record = ProofRecord1127 {
        id: 1127u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1128 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1128() -> u64 {
    let record = ProofRecord1128 {
        id: 1128u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1129 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1129() -> u64 {
    let record = ProofRecord1129 {
        id: 1129u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1130 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1130() -> u64 {
    let record = ProofRecord1130 {
        id: 1130u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1131 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1131() -> u64 {
    let record = ProofRecord1131 {
        id: 1131u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1132 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1132() -> u64 {
    let record = ProofRecord1132 {
        id: 1132u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1133 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1133() -> u64 {
    let record = ProofRecord1133 {
        id: 1133u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1134 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1134() -> u64 {
    let record = ProofRecord1134 {
        id: 1134u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1135 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1135() -> u64 {
    let record = ProofRecord1135 {
        id: 1135u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1136 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1136() -> u64 {
    let record = ProofRecord1136 {
        id: 1136u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1137 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1137() -> u64 {
    let record = ProofRecord1137 {
        id: 1137u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1138 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1138() -> u64 {
    let record = ProofRecord1138 {
        id: 1138u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1139 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1139() -> u64 {
    let record = ProofRecord1139 {
        id: 1139u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1140 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1140() -> u64 {
    let record = ProofRecord1140 {
        id: 1140u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1141 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1141() -> u64 {
    let record = ProofRecord1141 {
        id: 1141u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1142 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1142() -> u64 {
    let record = ProofRecord1142 {
        id: 1142u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1143 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1143() -> u64 {
    let record = ProofRecord1143 {
        id: 1143u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1144 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1144() -> u64 {
    let record = ProofRecord1144 {
        id: 1144u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1145 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1145() -> u64 {
    let record = ProofRecord1145 {
        id: 1145u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1146 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1146() -> u64 {
    let record = ProofRecord1146 {
        id: 1146u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1147 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1147() -> u64 {
    let record = ProofRecord1147 {
        id: 1147u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1148 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1148() -> u64 {
    let record = ProofRecord1148 {
        id: 1148u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1149 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1149() -> u64 {
    let record = ProofRecord1149 {
        id: 1149u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1150 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1150() -> u64 {
    let record = ProofRecord1150 {
        id: 1150u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1151 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1151() -> u64 {
    let record = ProofRecord1151 {
        id: 1151u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1152 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1152() -> u64 {
    let record = ProofRecord1152 {
        id: 1152u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1153 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1153() -> u64 {
    let record = ProofRecord1153 {
        id: 1153u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1154 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1154() -> u64 {
    let record = ProofRecord1154 {
        id: 1154u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1155 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1155() -> u64 {
    let record = ProofRecord1155 {
        id: 1155u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1156 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1156() -> u64 {
    let record = ProofRecord1156 {
        id: 1156u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1157 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1157() -> u64 {
    let record = ProofRecord1157 {
        id: 1157u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1158 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1158() -> u64 {
    let record = ProofRecord1158 {
        id: 1158u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1159 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1159() -> u64 {
    let record = ProofRecord1159 {
        id: 1159u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1160 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1160() -> u64 {
    let record = ProofRecord1160 {
        id: 1160u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1161 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1161() -> u64 {
    let record = ProofRecord1161 {
        id: 1161u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1162 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1162() -> u64 {
    let record = ProofRecord1162 {
        id: 1162u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1163 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1163() -> u64 {
    let record = ProofRecord1163 {
        id: 1163u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1164 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1164() -> u64 {
    let record = ProofRecord1164 {
        id: 1164u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1165 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1165() -> u64 {
    let record = ProofRecord1165 {
        id: 1165u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1166 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1166() -> u64 {
    let record = ProofRecord1166 {
        id: 1166u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1167 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1167() -> u64 {
    let record = ProofRecord1167 {
        id: 1167u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1168 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1168() -> u64 {
    let record = ProofRecord1168 {
        id: 1168u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1169 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1169() -> u64 {
    let record = ProofRecord1169 {
        id: 1169u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1170 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1170() -> u64 {
    let record = ProofRecord1170 {
        id: 1170u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1171 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1171() -> u64 {
    let record = ProofRecord1171 {
        id: 1171u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1172 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1172() -> u64 {
    let record = ProofRecord1172 {
        id: 1172u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1173 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1173() -> u64 {
    let record = ProofRecord1173 {
        id: 1173u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1174 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1174() -> u64 {
    let record = ProofRecord1174 {
        id: 1174u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1175 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1175() -> u64 {
    let record = ProofRecord1175 {
        id: 1175u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1176 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1176() -> u64 {
    let record = ProofRecord1176 {
        id: 1176u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1177 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1177() -> u64 {
    let record = ProofRecord1177 {
        id: 1177u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1178 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1178() -> u64 {
    let record = ProofRecord1178 {
        id: 1178u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1179 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1179() -> u64 {
    let record = ProofRecord1179 {
        id: 1179u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1180 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1180() -> u64 {
    let record = ProofRecord1180 {
        id: 1180u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1181 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1181() -> u64 {
    let record = ProofRecord1181 {
        id: 1181u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1182 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1182() -> u64 {
    let record = ProofRecord1182 {
        id: 1182u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1183 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1183() -> u64 {
    let record = ProofRecord1183 {
        id: 1183u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1184 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1184() -> u64 {
    let record = ProofRecord1184 {
        id: 1184u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1185 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1185() -> u64 {
    let record = ProofRecord1185 {
        id: 1185u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1186 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1186() -> u64 {
    let record = ProofRecord1186 {
        id: 1186u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1187 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1187() -> u64 {
    let record = ProofRecord1187 {
        id: 1187u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1188 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1188() -> u64 {
    let record = ProofRecord1188 {
        id: 1188u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1189 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1189() -> u64 {
    let record = ProofRecord1189 {
        id: 1189u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1190 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1190() -> u64 {
    let record = ProofRecord1190 {
        id: 1190u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1191 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1191() -> u64 {
    let record = ProofRecord1191 {
        id: 1191u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1192 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1192() -> u64 {
    let record = ProofRecord1192 {
        id: 1192u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1193 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1193() -> u64 {
    let record = ProofRecord1193 {
        id: 1193u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1194 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1194() -> u64 {
    let record = ProofRecord1194 {
        id: 1194u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1195 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1195() -> u64 {
    let record = ProofRecord1195 {
        id: 1195u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1196 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1196() -> u64 {
    let record = ProofRecord1196 {
        id: 1196u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1197 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1197() -> u64 {
    let record = ProofRecord1197 {
        id: 1197u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1198 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1198() -> u64 {
    let record = ProofRecord1198 {
        id: 1198u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1199 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1199() -> u64 {
    let record = ProofRecord1199 {
        id: 1199u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1200 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1200() -> u64 {
    let record = ProofRecord1200 {
        id: 1200u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1201 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1201() -> u64 {
    let record = ProofRecord1201 {
        id: 1201u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1202 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1202() -> u64 {
    let record = ProofRecord1202 {
        id: 1202u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1203 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1203() -> u64 {
    let record = ProofRecord1203 {
        id: 1203u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1204 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1204() -> u64 {
    let record = ProofRecord1204 {
        id: 1204u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1205 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1205() -> u64 {
    let record = ProofRecord1205 {
        id: 1205u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1206 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1206() -> u64 {
    let record = ProofRecord1206 {
        id: 1206u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1207 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1207() -> u64 {
    let record = ProofRecord1207 {
        id: 1207u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1208 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1208() -> u64 {
    let record = ProofRecord1208 {
        id: 1208u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1209 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1209() -> u64 {
    let record = ProofRecord1209 {
        id: 1209u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1210 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1210() -> u64 {
    let record = ProofRecord1210 {
        id: 1210u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1211 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1211() -> u64 {
    let record = ProofRecord1211 {
        id: 1211u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1212 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1212() -> u64 {
    let record = ProofRecord1212 {
        id: 1212u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1213 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1213() -> u64 {
    let record = ProofRecord1213 {
        id: 1213u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1214 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1214() -> u64 {
    let record = ProofRecord1214 {
        id: 1214u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1215 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1215() -> u64 {
    let record = ProofRecord1215 {
        id: 1215u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1216 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1216() -> u64 {
    let record = ProofRecord1216 {
        id: 1216u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1217 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1217() -> u64 {
    let record = ProofRecord1217 {
        id: 1217u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1218 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1218() -> u64 {
    let record = ProofRecord1218 {
        id: 1218u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1219 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1219() -> u64 {
    let record = ProofRecord1219 {
        id: 1219u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1220 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1220() -> u64 {
    let record = ProofRecord1220 {
        id: 1220u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1221 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1221() -> u64 {
    let record = ProofRecord1221 {
        id: 1221u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1222 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1222() -> u64 {
    let record = ProofRecord1222 {
        id: 1222u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1223 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1223() -> u64 {
    let record = ProofRecord1223 {
        id: 1223u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1224 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1224() -> u64 {
    let record = ProofRecord1224 {
        id: 1224u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1225 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1225() -> u64 {
    let record = ProofRecord1225 {
        id: 1225u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1226 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1226() -> u64 {
    let record = ProofRecord1226 {
        id: 1226u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1227 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1227() -> u64 {
    let record = ProofRecord1227 {
        id: 1227u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1228 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1228() -> u64 {
    let record = ProofRecord1228 {
        id: 1228u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1229 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1229() -> u64 {
    let record = ProofRecord1229 {
        id: 1229u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1230 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1230() -> u64 {
    let record = ProofRecord1230 {
        id: 1230u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1231 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1231() -> u64 {
    let record = ProofRecord1231 {
        id: 1231u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1232 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1232() -> u64 {
    let record = ProofRecord1232 {
        id: 1232u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1233 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1233() -> u64 {
    let record = ProofRecord1233 {
        id: 1233u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1234 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1234() -> u64 {
    let record = ProofRecord1234 {
        id: 1234u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1235 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1235() -> u64 {
    let record = ProofRecord1235 {
        id: 1235u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1236 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1236() -> u64 {
    let record = ProofRecord1236 {
        id: 1236u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1237 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1237() -> u64 {
    let record = ProofRecord1237 {
        id: 1237u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1238 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1238() -> u64 {
    let record = ProofRecord1238 {
        id: 1238u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1239 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1239() -> u64 {
    let record = ProofRecord1239 {
        id: 1239u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1240 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1240() -> u64 {
    let record = ProofRecord1240 {
        id: 1240u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1241 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1241() -> u64 {
    let record = ProofRecord1241 {
        id: 1241u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1242 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1242() -> u64 {
    let record = ProofRecord1242 {
        id: 1242u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1243 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1243() -> u64 {
    let record = ProofRecord1243 {
        id: 1243u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1244 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1244() -> u64 {
    let record = ProofRecord1244 {
        id: 1244u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1245 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1245() -> u64 {
    let record = ProofRecord1245 {
        id: 1245u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1246 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1246() -> u64 {
    let record = ProofRecord1246 {
        id: 1246u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1247 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1247() -> u64 {
    let record = ProofRecord1247 {
        id: 1247u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1248 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1248() -> u64 {
    let record = ProofRecord1248 {
        id: 1248u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1249 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1249() -> u64 {
    let record = ProofRecord1249 {
        id: 1249u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1250 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1250() -> u64 {
    let record = ProofRecord1250 {
        id: 1250u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1251 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1251() -> u64 {
    let record = ProofRecord1251 {
        id: 1251u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1252 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1252() -> u64 {
    let record = ProofRecord1252 {
        id: 1252u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1253 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1253() -> u64 {
    let record = ProofRecord1253 {
        id: 1253u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1254 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1254() -> u64 {
    let record = ProofRecord1254 {
        id: 1254u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1255 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1255() -> u64 {
    let record = ProofRecord1255 {
        id: 1255u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1256 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1256() -> u64 {
    let record = ProofRecord1256 {
        id: 1256u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1257 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1257() -> u64 {
    let record = ProofRecord1257 {
        id: 1257u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1258 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1258() -> u64 {
    let record = ProofRecord1258 {
        id: 1258u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1259 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1259() -> u64 {
    let record = ProofRecord1259 {
        id: 1259u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1260 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1260() -> u64 {
    let record = ProofRecord1260 {
        id: 1260u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1261 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1261() -> u64 {
    let record = ProofRecord1261 {
        id: 1261u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1262 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1262() -> u64 {
    let record = ProofRecord1262 {
        id: 1262u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1263 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1263() -> u64 {
    let record = ProofRecord1263 {
        id: 1263u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1264 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1264() -> u64 {
    let record = ProofRecord1264 {
        id: 1264u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1265 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1265() -> u64 {
    let record = ProofRecord1265 {
        id: 1265u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1266 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1266() -> u64 {
    let record = ProofRecord1266 {
        id: 1266u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1267 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1267() -> u64 {
    let record = ProofRecord1267 {
        id: 1267u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1268 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1268() -> u64 {
    let record = ProofRecord1268 {
        id: 1268u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1269 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1269() -> u64 {
    let record = ProofRecord1269 {
        id: 1269u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1270 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1270() -> u64 {
    let record = ProofRecord1270 {
        id: 1270u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1271 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1271() -> u64 {
    let record = ProofRecord1271 {
        id: 1271u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1272 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1272() -> u64 {
    let record = ProofRecord1272 {
        id: 1272u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1273 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1273() -> u64 {
    let record = ProofRecord1273 {
        id: 1273u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1274 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1274() -> u64 {
    let record = ProofRecord1274 {
        id: 1274u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1275 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1275() -> u64 {
    let record = ProofRecord1275 {
        id: 1275u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1276 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1276() -> u64 {
    let record = ProofRecord1276 {
        id: 1276u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1277 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1277() -> u64 {
    let record = ProofRecord1277 {
        id: 1277u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1278 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1278() -> u64 {
    let record = ProofRecord1278 {
        id: 1278u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1279 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1279() -> u64 {
    let record = ProofRecord1279 {
        id: 1279u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1280 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1280() -> u64 {
    let record = ProofRecord1280 {
        id: 1280u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1281 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1281() -> u64 {
    let record = ProofRecord1281 {
        id: 1281u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1282 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1282() -> u64 {
    let record = ProofRecord1282 {
        id: 1282u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1283 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1283() -> u64 {
    let record = ProofRecord1283 {
        id: 1283u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1284 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1284() -> u64 {
    let record = ProofRecord1284 {
        id: 1284u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1285 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1285() -> u64 {
    let record = ProofRecord1285 {
        id: 1285u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1286 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1286() -> u64 {
    let record = ProofRecord1286 {
        id: 1286u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1287 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1287() -> u64 {
    let record = ProofRecord1287 {
        id: 1287u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1288 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1288() -> u64 {
    let record = ProofRecord1288 {
        id: 1288u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1289 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1289() -> u64 {
    let record = ProofRecord1289 {
        id: 1289u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1290 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1290() -> u64 {
    let record = ProofRecord1290 {
        id: 1290u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1291 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1291() -> u64 {
    let record = ProofRecord1291 {
        id: 1291u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1292 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1292() -> u64 {
    let record = ProofRecord1292 {
        id: 1292u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1293 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1293() -> u64 {
    let record = ProofRecord1293 {
        id: 1293u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1294 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1294() -> u64 {
    let record = ProofRecord1294 {
        id: 1294u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1295 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1295() -> u64 {
    let record = ProofRecord1295 {
        id: 1295u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1296 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1296() -> u64 {
    let record = ProofRecord1296 {
        id: 1296u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1297 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1297() -> u64 {
    let record = ProofRecord1297 {
        id: 1297u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1298 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1298() -> u64 {
    let record = ProofRecord1298 {
        id: 1298u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1299 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1299() -> u64 {
    let record = ProofRecord1299 {
        id: 1299u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1300 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1300() -> u64 {
    let record = ProofRecord1300 {
        id: 1300u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1301 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1301() -> u64 {
    let record = ProofRecord1301 {
        id: 1301u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1302 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1302() -> u64 {
    let record = ProofRecord1302 {
        id: 1302u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1303 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1303() -> u64 {
    let record = ProofRecord1303 {
        id: 1303u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1304 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1304() -> u64 {
    let record = ProofRecord1304 {
        id: 1304u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1305 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1305() -> u64 {
    let record = ProofRecord1305 {
        id: 1305u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1306 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1306() -> u64 {
    let record = ProofRecord1306 {
        id: 1306u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1307 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1307() -> u64 {
    let record = ProofRecord1307 {
        id: 1307u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1308 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1308() -> u64 {
    let record = ProofRecord1308 {
        id: 1308u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1309 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1309() -> u64 {
    let record = ProofRecord1309 {
        id: 1309u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1310 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1310() -> u64 {
    let record = ProofRecord1310 {
        id: 1310u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1311 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1311() -> u64 {
    let record = ProofRecord1311 {
        id: 1311u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1312 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1312() -> u64 {
    let record = ProofRecord1312 {
        id: 1312u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1313 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1313() -> u64 {
    let record = ProofRecord1313 {
        id: 1313u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1314 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1314() -> u64 {
    let record = ProofRecord1314 {
        id: 1314u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1315 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1315() -> u64 {
    let record = ProofRecord1315 {
        id: 1315u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1316 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1316() -> u64 {
    let record = ProofRecord1316 {
        id: 1316u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1317 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1317() -> u64 {
    let record = ProofRecord1317 {
        id: 1317u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1318 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1318() -> u64 {
    let record = ProofRecord1318 {
        id: 1318u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1319 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1319() -> u64 {
    let record = ProofRecord1319 {
        id: 1319u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1320 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1320() -> u64 {
    let record = ProofRecord1320 {
        id: 1320u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1321 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1321() -> u64 {
    let record = ProofRecord1321 {
        id: 1321u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1322 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1322() -> u64 {
    let record = ProofRecord1322 {
        id: 1322u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1323 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1323() -> u64 {
    let record = ProofRecord1323 {
        id: 1323u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1324 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1324() -> u64 {
    let record = ProofRecord1324 {
        id: 1324u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1325 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1325() -> u64 {
    let record = ProofRecord1325 {
        id: 1325u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1326 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1326() -> u64 {
    let record = ProofRecord1326 {
        id: 1326u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1327 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1327() -> u64 {
    let record = ProofRecord1327 {
        id: 1327u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1328 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1328() -> u64 {
    let record = ProofRecord1328 {
        id: 1328u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1329 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1329() -> u64 {
    let record = ProofRecord1329 {
        id: 1329u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1330 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1330() -> u64 {
    let record = ProofRecord1330 {
        id: 1330u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1331 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1331() -> u64 {
    let record = ProofRecord1331 {
        id: 1331u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1332 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1332() -> u64 {
    let record = ProofRecord1332 {
        id: 1332u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1333 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1333() -> u64 {
    let record = ProofRecord1333 {
        id: 1333u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1334 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1334() -> u64 {
    let record = ProofRecord1334 {
        id: 1334u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1335 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1335() -> u64 {
    let record = ProofRecord1335 {
        id: 1335u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1336 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1336() -> u64 {
    let record = ProofRecord1336 {
        id: 1336u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1337 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1337() -> u64 {
    let record = ProofRecord1337 {
        id: 1337u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1338 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1338() -> u64 {
    let record = ProofRecord1338 {
        id: 1338u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1339 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1339() -> u64 {
    let record = ProofRecord1339 {
        id: 1339u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1340 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1340() -> u64 {
    let record = ProofRecord1340 {
        id: 1340u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1341 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1341() -> u64 {
    let record = ProofRecord1341 {
        id: 1341u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1342 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1342() -> u64 {
    let record = ProofRecord1342 {
        id: 1342u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1343 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1343() -> u64 {
    let record = ProofRecord1343 {
        id: 1343u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1344 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1344() -> u64 {
    let record = ProofRecord1344 {
        id: 1344u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1345 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1345() -> u64 {
    let record = ProofRecord1345 {
        id: 1345u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1346 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1346() -> u64 {
    let record = ProofRecord1346 {
        id: 1346u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1347 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1347() -> u64 {
    let record = ProofRecord1347 {
        id: 1347u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1348 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1348() -> u64 {
    let record = ProofRecord1348 {
        id: 1348u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1349 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1349() -> u64 {
    let record = ProofRecord1349 {
        id: 1349u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1350 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1350() -> u64 {
    let record = ProofRecord1350 {
        id: 1350u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1351 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1351() -> u64 {
    let record = ProofRecord1351 {
        id: 1351u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1352 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1352() -> u64 {
    let record = ProofRecord1352 {
        id: 1352u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1353 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1353() -> u64 {
    let record = ProofRecord1353 {
        id: 1353u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1354 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1354() -> u64 {
    let record = ProofRecord1354 {
        id: 1354u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1355 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1355() -> u64 {
    let record = ProofRecord1355 {
        id: 1355u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1356 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1356() -> u64 {
    let record = ProofRecord1356 {
        id: 1356u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1357 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1357() -> u64 {
    let record = ProofRecord1357 {
        id: 1357u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1358 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1358() -> u64 {
    let record = ProofRecord1358 {
        id: 1358u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1359 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1359() -> u64 {
    let record = ProofRecord1359 {
        id: 1359u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1360 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1360() -> u64 {
    let record = ProofRecord1360 {
        id: 1360u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1361 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1361() -> u64 {
    let record = ProofRecord1361 {
        id: 1361u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1362 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1362() -> u64 {
    let record = ProofRecord1362 {
        id: 1362u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1363 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1363() -> u64 {
    let record = ProofRecord1363 {
        id: 1363u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1364 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1364() -> u64 {
    let record = ProofRecord1364 {
        id: 1364u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1365 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1365() -> u64 {
    let record = ProofRecord1365 {
        id: 1365u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1366 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1366() -> u64 {
    let record = ProofRecord1366 {
        id: 1366u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1367 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1367() -> u64 {
    let record = ProofRecord1367 {
        id: 1367u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1368 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1368() -> u64 {
    let record = ProofRecord1368 {
        id: 1368u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1369 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1369() -> u64 {
    let record = ProofRecord1369 {
        id: 1369u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1370 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1370() -> u64 {
    let record = ProofRecord1370 {
        id: 1370u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1371 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1371() -> u64 {
    let record = ProofRecord1371 {
        id: 1371u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1372 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1372() -> u64 {
    let record = ProofRecord1372 {
        id: 1372u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1373 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1373() -> u64 {
    let record = ProofRecord1373 {
        id: 1373u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1374 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1374() -> u64 {
    let record = ProofRecord1374 {
        id: 1374u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1375 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1375() -> u64 {
    let record = ProofRecord1375 {
        id: 1375u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1376 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1376() -> u64 {
    let record = ProofRecord1376 {
        id: 1376u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1377 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1377() -> u64 {
    let record = ProofRecord1377 {
        id: 1377u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1378 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1378() -> u64 {
    let record = ProofRecord1378 {
        id: 1378u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1379 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1379() -> u64 {
    let record = ProofRecord1379 {
        id: 1379u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1380 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1380() -> u64 {
    let record = ProofRecord1380 {
        id: 1380u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1381 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1381() -> u64 {
    let record = ProofRecord1381 {
        id: 1381u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1382 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1382() -> u64 {
    let record = ProofRecord1382 {
        id: 1382u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1383 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1383() -> u64 {
    let record = ProofRecord1383 {
        id: 1383u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1384 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1384() -> u64 {
    let record = ProofRecord1384 {
        id: 1384u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1385 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1385() -> u64 {
    let record = ProofRecord1385 {
        id: 1385u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1386 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1386() -> u64 {
    let record = ProofRecord1386 {
        id: 1386u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1387 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1387() -> u64 {
    let record = ProofRecord1387 {
        id: 1387u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1388 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1388() -> u64 {
    let record = ProofRecord1388 {
        id: 1388u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1389 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1389() -> u64 {
    let record = ProofRecord1389 {
        id: 1389u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1390 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1390() -> u64 {
    let record = ProofRecord1390 {
        id: 1390u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1391 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1391() -> u64 {
    let record = ProofRecord1391 {
        id: 1391u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1392 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1392() -> u64 {
    let record = ProofRecord1392 {
        id: 1392u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1393 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1393() -> u64 {
    let record = ProofRecord1393 {
        id: 1393u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1394 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1394() -> u64 {
    let record = ProofRecord1394 {
        id: 1394u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1395 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1395() -> u64 {
    let record = ProofRecord1395 {
        id: 1395u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1396 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1396() -> u64 {
    let record = ProofRecord1396 {
        id: 1396u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1397 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1397() -> u64 {
    let record = ProofRecord1397 {
        id: 1397u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1398 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1398() -> u64 {
    let record = ProofRecord1398 {
        id: 1398u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1399 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1399() -> u64 {
    let record = ProofRecord1399 {
        id: 1399u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1400 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1400() -> u64 {
    let record = ProofRecord1400 {
        id: 1400u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1401 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1401() -> u64 {
    let record = ProofRecord1401 {
        id: 1401u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1402 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1402() -> u64 {
    let record = ProofRecord1402 {
        id: 1402u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1403 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1403() -> u64 {
    let record = ProofRecord1403 {
        id: 1403u64,
        label: "cinder-proof",
        values: [46, 47, 48, 49, 50, 51, 52, 53],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1404 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1404() -> u64 {
    let record = ProofRecord1404 {
        id: 1404u64,
        label: "cinder-proof",
        values: [47, 48, 49, 50, 51, 52, 53, 54],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1405 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1405() -> u64 {
    let record = ProofRecord1405 {
        id: 1405u64,
        label: "cinder-proof",
        values: [48, 49, 50, 51, 52, 53, 54, 55],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1406 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1406() -> u64 {
    let record = ProofRecord1406 {
        id: 1406u64,
        label: "cinder-proof",
        values: [49, 50, 51, 52, 53, 54, 55, 56],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1407 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1407() -> u64 {
    let record = ProofRecord1407 {
        id: 1407u64,
        label: "cinder-proof",
        values: [50, 51, 52, 53, 54, 55, 56, 57],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1408 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1408() -> u64 {
    let record = ProofRecord1408 {
        id: 1408u64,
        label: "cinder-proof",
        values: [51, 52, 53, 54, 55, 56, 57, 58],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1409 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1409() -> u64 {
    let record = ProofRecord1409 {
        id: 1409u64,
        label: "cinder-proof",
        values: [52, 53, 54, 55, 56, 57, 58, 59],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1410 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1410() -> u64 {
    let record = ProofRecord1410 {
        id: 1410u64,
        label: "cinder-proof",
        values: [53, 54, 55, 56, 57, 58, 59, 60],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1411 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1411() -> u64 {
    let record = ProofRecord1411 {
        id: 1411u64,
        label: "cinder-proof",
        values: [54, 55, 56, 57, 58, 59, 60, 61],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1412 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1412() -> u64 {
    let record = ProofRecord1412 {
        id: 1412u64,
        label: "cinder-proof",
        values: [55, 56, 57, 58, 59, 60, 61, 62],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1413 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1413() -> u64 {
    let record = ProofRecord1413 {
        id: 1413u64,
        label: "cinder-proof",
        values: [56, 57, 58, 59, 60, 61, 62, 63],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1414 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1414() -> u64 {
    let record = ProofRecord1414 {
        id: 1414u64,
        label: "cinder-proof",
        values: [57, 58, 59, 60, 61, 62, 63, 64],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1415 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1415() -> u64 {
    let record = ProofRecord1415 {
        id: 1415u64,
        label: "cinder-proof",
        values: [58, 59, 60, 61, 62, 63, 64, 65],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1416 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1416() -> u64 {
    let record = ProofRecord1416 {
        id: 1416u64,
        label: "cinder-proof",
        values: [59, 60, 61, 62, 63, 64, 65, 66],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1417 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1417() -> u64 {
    let record = ProofRecord1417 {
        id: 1417u64,
        label: "cinder-proof",
        values: [60, 61, 62, 63, 64, 65, 66, 67],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1418 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1418() -> u64 {
    let record = ProofRecord1418 {
        id: 1418u64,
        label: "cinder-proof",
        values: [61, 62, 63, 64, 65, 66, 67, 68],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1419 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1419() -> u64 {
    let record = ProofRecord1419 {
        id: 1419u64,
        label: "cinder-proof",
        values: [62, 63, 64, 65, 66, 67, 68, 69],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1420 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1420() -> u64 {
    let record = ProofRecord1420 {
        id: 1420u64,
        label: "cinder-proof",
        values: [63, 64, 65, 66, 67, 68, 69, 70],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1421 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1421() -> u64 {
    let record = ProofRecord1421 {
        id: 1421u64,
        label: "cinder-proof",
        values: [64, 65, 66, 67, 68, 69, 70, 71],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1422 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1422() -> u64 {
    let record = ProofRecord1422 {
        id: 1422u64,
        label: "cinder-proof",
        values: [65, 66, 67, 68, 69, 70, 71, 72],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1423 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1423() -> u64 {
    let record = ProofRecord1423 {
        id: 1423u64,
        label: "cinder-proof",
        values: [66, 67, 68, 69, 70, 71, 72, 73],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1424 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1424() -> u64 {
    let record = ProofRecord1424 {
        id: 1424u64,
        label: "cinder-proof",
        values: [67, 68, 69, 70, 71, 72, 73, 74],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1425 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1425() -> u64 {
    let record = ProofRecord1425 {
        id: 1425u64,
        label: "cinder-proof",
        values: [68, 69, 70, 71, 72, 73, 74, 75],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1426 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1426() -> u64 {
    let record = ProofRecord1426 {
        id: 1426u64,
        label: "cinder-proof",
        values: [69, 70, 71, 72, 73, 74, 75, 76],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1427 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1427() -> u64 {
    let record = ProofRecord1427 {
        id: 1427u64,
        label: "cinder-proof",
        values: [70, 71, 72, 73, 74, 75, 76, 77],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1428 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1428() -> u64 {
    let record = ProofRecord1428 {
        id: 1428u64,
        label: "cinder-proof",
        values: [71, 72, 73, 74, 75, 76, 77, 78],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1429 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1429() -> u64 {
    let record = ProofRecord1429 {
        id: 1429u64,
        label: "cinder-proof",
        values: [72, 73, 74, 75, 76, 77, 78, 79],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1430 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1430() -> u64 {
    let record = ProofRecord1430 {
        id: 1430u64,
        label: "cinder-proof",
        values: [73, 74, 75, 76, 77, 78, 79, 80],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1431 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1431() -> u64 {
    let record = ProofRecord1431 {
        id: 1431u64,
        label: "cinder-proof",
        values: [74, 75, 76, 77, 78, 79, 80, 81],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1432 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1432() -> u64 {
    let record = ProofRecord1432 {
        id: 1432u64,
        label: "cinder-proof",
        values: [75, 76, 77, 78, 79, 80, 81, 82],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1433 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1433() -> u64 {
    let record = ProofRecord1433 {
        id: 1433u64,
        label: "cinder-proof",
        values: [76, 77, 78, 79, 80, 81, 82, 83],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1434 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1434() -> u64 {
    let record = ProofRecord1434 {
        id: 1434u64,
        label: "cinder-proof",
        values: [77, 78, 79, 80, 81, 82, 83, 84],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1435 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1435() -> u64 {
    let record = ProofRecord1435 {
        id: 1435u64,
        label: "cinder-proof",
        values: [78, 79, 80, 81, 82, 83, 84, 85],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1436 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1436() -> u64 {
    let record = ProofRecord1436 {
        id: 1436u64,
        label: "cinder-proof",
        values: [79, 80, 81, 82, 83, 84, 85, 86],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1437 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1437() -> u64 {
    let record = ProofRecord1437 {
        id: 1437u64,
        label: "cinder-proof",
        values: [80, 81, 82, 83, 84, 85, 86, 87],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1438 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1438() -> u64 {
    let record = ProofRecord1438 {
        id: 1438u64,
        label: "cinder-proof",
        values: [81, 82, 83, 84, 85, 86, 87, 88],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1439 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1439() -> u64 {
    let record = ProofRecord1439 {
        id: 1439u64,
        label: "cinder-proof",
        values: [82, 83, 84, 85, 86, 87, 88, 89],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1440 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1440() -> u64 {
    let record = ProofRecord1440 {
        id: 1440u64,
        label: "cinder-proof",
        values: [83, 84, 85, 86, 87, 88, 89, 90],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1441 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1441() -> u64 {
    let record = ProofRecord1441 {
        id: 1441u64,
        label: "cinder-proof",
        values: [84, 85, 86, 87, 88, 89, 90, 91],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1442 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1442() -> u64 {
    let record = ProofRecord1442 {
        id: 1442u64,
        label: "cinder-proof",
        values: [85, 86, 87, 88, 89, 90, 91, 92],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1443 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1443() -> u64 {
    let record = ProofRecord1443 {
        id: 1443u64,
        label: "cinder-proof",
        values: [86, 87, 88, 89, 90, 91, 92, 93],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1444 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1444() -> u64 {
    let record = ProofRecord1444 {
        id: 1444u64,
        label: "cinder-proof",
        values: [87, 88, 89, 90, 91, 92, 93, 94],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1445 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1445() -> u64 {
    let record = ProofRecord1445 {
        id: 1445u64,
        label: "cinder-proof",
        values: [88, 89, 90, 91, 92, 93, 94, 95],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1446 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1446() -> u64 {
    let record = ProofRecord1446 {
        id: 1446u64,
        label: "cinder-proof",
        values: [89, 90, 91, 92, 93, 94, 95, 96],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1447 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1447() -> u64 {
    let record = ProofRecord1447 {
        id: 1447u64,
        label: "cinder-proof",
        values: [90, 91, 92, 93, 94, 95, 96, 97],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1448 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1448() -> u64 {
    let record = ProofRecord1448 {
        id: 1448u64,
        label: "cinder-proof",
        values: [91, 92, 93, 94, 95, 96, 97, 98],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1449 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1449() -> u64 {
    let record = ProofRecord1449 {
        id: 1449u64,
        label: "cinder-proof",
        values: [92, 93, 94, 95, 96, 97, 98, 99],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1450 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1450() -> u64 {
    let record = ProofRecord1450 {
        id: 1450u64,
        label: "cinder-proof",
        values: [93, 94, 95, 96, 97, 98, 99, 100],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1451 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1451() -> u64 {
    let record = ProofRecord1451 {
        id: 1451u64,
        label: "cinder-proof",
        values: [94, 95, 96, 97, 98, 99, 100, 101],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1452 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1452() -> u64 {
    let record = ProofRecord1452 {
        id: 1452u64,
        label: "cinder-proof",
        values: [95, 96, 97, 98, 99, 100, 101, 102],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1453 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1453() -> u64 {
    let record = ProofRecord1453 {
        id: 1453u64,
        label: "cinder-proof",
        values: [96, 97, 98, 99, 100, 101, 102, 103],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1454 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1454() -> u64 {
    let record = ProofRecord1454 {
        id: 1454u64,
        label: "cinder-proof",
        values: [97, 98, 99, 100, 101, 102, 103, 104],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1455 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1455() -> u64 {
    let record = ProofRecord1455 {
        id: 1455u64,
        label: "cinder-proof",
        values: [1, 2, 3, 4, 5, 6, 7, 8],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1456 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1456() -> u64 {
    let record = ProofRecord1456 {
        id: 1456u64,
        label: "cinder-proof",
        values: [2, 3, 4, 5, 6, 7, 8, 9],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1457 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1457() -> u64 {
    let record = ProofRecord1457 {
        id: 1457u64,
        label: "cinder-proof",
        values: [3, 4, 5, 6, 7, 8, 9, 10],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1458 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1458() -> u64 {
    let record = ProofRecord1458 {
        id: 1458u64,
        label: "cinder-proof",
        values: [4, 5, 6, 7, 8, 9, 10, 11],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1459 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1459() -> u64 {
    let record = ProofRecord1459 {
        id: 1459u64,
        label: "cinder-proof",
        values: [5, 6, 7, 8, 9, 10, 11, 12],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1460 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1460() -> u64 {
    let record = ProofRecord1460 {
        id: 1460u64,
        label: "cinder-proof",
        values: [6, 7, 8, 9, 10, 11, 12, 13],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1461 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1461() -> u64 {
    let record = ProofRecord1461 {
        id: 1461u64,
        label: "cinder-proof",
        values: [7, 8, 9, 10, 11, 12, 13, 14],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1462 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1462() -> u64 {
    let record = ProofRecord1462 {
        id: 1462u64,
        label: "cinder-proof",
        values: [8, 9, 10, 11, 12, 13, 14, 15],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1463 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1463() -> u64 {
    let record = ProofRecord1463 {
        id: 1463u64,
        label: "cinder-proof",
        values: [9, 10, 11, 12, 13, 14, 15, 16],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1464 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1464() -> u64 {
    let record = ProofRecord1464 {
        id: 1464u64,
        label: "cinder-proof",
        values: [10, 11, 12, 13, 14, 15, 16, 17],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1465 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1465() -> u64 {
    let record = ProofRecord1465 {
        id: 1465u64,
        label: "cinder-proof",
        values: [11, 12, 13, 14, 15, 16, 17, 18],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1466 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1466() -> u64 {
    let record = ProofRecord1466 {
        id: 1466u64,
        label: "cinder-proof",
        values: [12, 13, 14, 15, 16, 17, 18, 19],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1467 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1467() -> u64 {
    let record = ProofRecord1467 {
        id: 1467u64,
        label: "cinder-proof",
        values: [13, 14, 15, 16, 17, 18, 19, 20],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1468 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1468() -> u64 {
    let record = ProofRecord1468 {
        id: 1468u64,
        label: "cinder-proof",
        values: [14, 15, 16, 17, 18, 19, 20, 21],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1469 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1469() -> u64 {
    let record = ProofRecord1469 {
        id: 1469u64,
        label: "cinder-proof",
        values: [15, 16, 17, 18, 19, 20, 21, 22],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1470 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1470() -> u64 {
    let record = ProofRecord1470 {
        id: 1470u64,
        label: "cinder-proof",
        values: [16, 17, 18, 19, 20, 21, 22, 23],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1471 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1471() -> u64 {
    let record = ProofRecord1471 {
        id: 1471u64,
        label: "cinder-proof",
        values: [17, 18, 19, 20, 21, 22, 23, 24],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1472 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1472() -> u64 {
    let record = ProofRecord1472 {
        id: 1472u64,
        label: "cinder-proof",
        values: [18, 19, 20, 21, 22, 23, 24, 25],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1473 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1473() -> u64 {
    let record = ProofRecord1473 {
        id: 1473u64,
        label: "cinder-proof",
        values: [19, 20, 21, 22, 23, 24, 25, 26],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1474 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1474() -> u64 {
    let record = ProofRecord1474 {
        id: 1474u64,
        label: "cinder-proof",
        values: [20, 21, 22, 23, 24, 25, 26, 27],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1475 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1475() -> u64 {
    let record = ProofRecord1475 {
        id: 1475u64,
        label: "cinder-proof",
        values: [21, 22, 23, 24, 25, 26, 27, 28],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1476 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1476() -> u64 {
    let record = ProofRecord1476 {
        id: 1476u64,
        label: "cinder-proof",
        values: [22, 23, 24, 25, 26, 27, 28, 29],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1477 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1477() -> u64 {
    let record = ProofRecord1477 {
        id: 1477u64,
        label: "cinder-proof",
        values: [23, 24, 25, 26, 27, 28, 29, 30],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1478 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1478() -> u64 {
    let record = ProofRecord1478 {
        id: 1478u64,
        label: "cinder-proof",
        values: [24, 25, 26, 27, 28, 29, 30, 31],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1479 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1479() -> u64 {
    let record = ProofRecord1479 {
        id: 1479u64,
        label: "cinder-proof",
        values: [25, 26, 27, 28, 29, 30, 31, 32],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1480 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1480() -> u64 {
    let record = ProofRecord1480 {
        id: 1480u64,
        label: "cinder-proof",
        values: [26, 27, 28, 29, 30, 31, 32, 33],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1481 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1481() -> u64 {
    let record = ProofRecord1481 {
        id: 1481u64,
        label: "cinder-proof",
        values: [27, 28, 29, 30, 31, 32, 33, 34],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1482 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1482() -> u64 {
    let record = ProofRecord1482 {
        id: 1482u64,
        label: "cinder-proof",
        values: [28, 29, 30, 31, 32, 33, 34, 35],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1483 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1483() -> u64 {
    let record = ProofRecord1483 {
        id: 1483u64,
        label: "cinder-proof",
        values: [29, 30, 31, 32, 33, 34, 35, 36],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1484 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1484() -> u64 {
    let record = ProofRecord1484 {
        id: 1484u64,
        label: "cinder-proof",
        values: [30, 31, 32, 33, 34, 35, 36, 37],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1485 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1485() -> u64 {
    let record = ProofRecord1485 {
        id: 1485u64,
        label: "cinder-proof",
        values: [31, 32, 33, 34, 35, 36, 37, 38],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1486 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1486() -> u64 {
    let record = ProofRecord1486 {
        id: 1486u64,
        label: "cinder-proof",
        values: [32, 33, 34, 35, 36, 37, 38, 39],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1487 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1487() -> u64 {
    let record = ProofRecord1487 {
        id: 1487u64,
        label: "cinder-proof",
        values: [33, 34, 35, 36, 37, 38, 39, 40],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1488 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1488() -> u64 {
    let record = ProofRecord1488 {
        id: 1488u64,
        label: "cinder-proof",
        values: [34, 35, 36, 37, 38, 39, 40, 41],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1489 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1489() -> u64 {
    let record = ProofRecord1489 {
        id: 1489u64,
        label: "cinder-proof",
        values: [35, 36, 37, 38, 39, 40, 41, 42],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1490 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1490() -> u64 {
    let record = ProofRecord1490 {
        id: 1490u64,
        label: "cinder-proof",
        values: [36, 37, 38, 39, 40, 41, 42, 43],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1491 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1491() -> u64 {
    let record = ProofRecord1491 {
        id: 1491u64,
        label: "cinder-proof",
        values: [37, 38, 39, 40, 41, 42, 43, 44],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1492 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1492() -> u64 {
    let record = ProofRecord1492 {
        id: 1492u64,
        label: "cinder-proof",
        values: [38, 39, 40, 41, 42, 43, 44, 45],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1493 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1493() -> u64 {
    let record = ProofRecord1493 {
        id: 1493u64,
        label: "cinder-proof",
        values: [39, 40, 41, 42, 43, 44, 45, 46],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1494 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1494() -> u64 {
    let record = ProofRecord1494 {
        id: 1494u64,
        label: "cinder-proof",
        values: [40, 41, 42, 43, 44, 45, 46, 47],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1495 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1495() -> u64 {
    let record = ProofRecord1495 {
        id: 1495u64,
        label: "cinder-proof",
        values: [41, 42, 43, 44, 45, 46, 47, 48],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1496 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1496() -> u64 {
    let record = ProofRecord1496 {
        id: 1496u64,
        label: "cinder-proof",
        values: [42, 43, 44, 45, 46, 47, 48, 49],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1497 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1497() -> u64 {
    let record = ProofRecord1497 {
        id: 1497u64,
        label: "cinder-proof",
        values: [43, 44, 45, 46, 47, 48, 49, 50],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1498 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1498() -> u64 {
    let record = ProofRecord1498 {
        id: 1498u64,
        label: "cinder-proof",
        values: [44, 45, 46, 47, 48, 49, 50, 51],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

#[derive(Serialize, Deserialize)]
struct ProofRecord1499 {
    id: u64,
    label: &'static str,
    values: [u32; 8],
}

fn proof_record_1499() -> u64 {
    let record = ProofRecord1499 {
        id: 1499u64,
        label: "cinder-proof",
        values: [45, 46, 47, 48, 49, 50, 51, 52],
    };

    record.id + record.values.iter().map(|value| *value as u64).sum::<u64>()
}

fn main() {
    let checksum: u64 = proof_record_0() + proof_record_1() + proof_record_2() + proof_record_3() + proof_record_4() + proof_record_5() + proof_record_6() + proof_record_7() + proof_record_8() + proof_record_9() + proof_record_10() + proof_record_11() + proof_record_12() + proof_record_13() + proof_record_14() + proof_record_15() + proof_record_16() + proof_record_17() + proof_record_18() + proof_record_19() + proof_record_20() + proof_record_21() + proof_record_22() + proof_record_23() + proof_record_24() + proof_record_25() + proof_record_26() + proof_record_27() + proof_record_28() + proof_record_29() + proof_record_30() + proof_record_31() + proof_record_32() + proof_record_33() + proof_record_34() + proof_record_35() + proof_record_36() + proof_record_37() + proof_record_38() + proof_record_39() + proof_record_40() + proof_record_41() + proof_record_42() + proof_record_43() + proof_record_44() + proof_record_45() + proof_record_46() + proof_record_47() + proof_record_48() + proof_record_49() + proof_record_50() + proof_record_51() + proof_record_52() + proof_record_53() + proof_record_54() + proof_record_55() + proof_record_56() + proof_record_57() + proof_record_58() + proof_record_59() + proof_record_60() + proof_record_61() + proof_record_62() + proof_record_63() + proof_record_64() + proof_record_65() + proof_record_66() + proof_record_67() + proof_record_68() + proof_record_69() + proof_record_70() + proof_record_71() + proof_record_72() + proof_record_73() + proof_record_74() + proof_record_75() + proof_record_76() + proof_record_77() + proof_record_78() + proof_record_79() + proof_record_80() + proof_record_81() + proof_record_82() + proof_record_83() + proof_record_84() + proof_record_85() + proof_record_86() + proof_record_87() + proof_record_88() + proof_record_89() + proof_record_90() + proof_record_91() + proof_record_92() + proof_record_93() + proof_record_94() + proof_record_95() + proof_record_96() + proof_record_97() + proof_record_98() + proof_record_99() + proof_record_100() + proof_record_101() + proof_record_102() + proof_record_103() + proof_record_104() + proof_record_105() + proof_record_106() + proof_record_107() + proof_record_108() + proof_record_109() + proof_record_110() + proof_record_111() + proof_record_112() + proof_record_113() + proof_record_114() + proof_record_115() + proof_record_116() + proof_record_117() + proof_record_118() + proof_record_119() + proof_record_120() + proof_record_121() + proof_record_122() + proof_record_123() + proof_record_124() + proof_record_125() + proof_record_126() + proof_record_127() + proof_record_128() + proof_record_129() + proof_record_130() + proof_record_131() + proof_record_132() + proof_record_133() + proof_record_134() + proof_record_135() + proof_record_136() + proof_record_137() + proof_record_138() + proof_record_139() + proof_record_140() + proof_record_141() + proof_record_142() + proof_record_143() + proof_record_144() + proof_record_145() + proof_record_146() + proof_record_147() + proof_record_148() + proof_record_149() + proof_record_150() + proof_record_151() + proof_record_152() + proof_record_153() + proof_record_154() + proof_record_155() + proof_record_156() + proof_record_157() + proof_record_158() + proof_record_159() + proof_record_160() + proof_record_161() + proof_record_162() + proof_record_163() + proof_record_164() + proof_record_165() + proof_record_166() + proof_record_167() + proof_record_168() + proof_record_169() + proof_record_170() + proof_record_171() + proof_record_172() + proof_record_173() + proof_record_174() + proof_record_175() + proof_record_176() + proof_record_177() + proof_record_178() + proof_record_179() + proof_record_180() + proof_record_181() + proof_record_182() + proof_record_183() + proof_record_184() + proof_record_185() + proof_record_186() + proof_record_187() + proof_record_188() + proof_record_189() + proof_record_190() + proof_record_191() + proof_record_192() + proof_record_193() + proof_record_194() + proof_record_195() + proof_record_196() + proof_record_197() + proof_record_198() + proof_record_199() + proof_record_200() + proof_record_201() + proof_record_202() + proof_record_203() + proof_record_204() + proof_record_205() + proof_record_206() + proof_record_207() + proof_record_208() + proof_record_209() + proof_record_210() + proof_record_211() + proof_record_212() + proof_record_213() + proof_record_214() + proof_record_215() + proof_record_216() + proof_record_217() + proof_record_218() + proof_record_219() + proof_record_220() + proof_record_221() + proof_record_222() + proof_record_223() + proof_record_224() + proof_record_225() + proof_record_226() + proof_record_227() + proof_record_228() + proof_record_229() + proof_record_230() + proof_record_231() + proof_record_232() + proof_record_233() + proof_record_234() + proof_record_235() + proof_record_236() + proof_record_237() + proof_record_238() + proof_record_239() + proof_record_240() + proof_record_241() + proof_record_242() + proof_record_243() + proof_record_244() + proof_record_245() + proof_record_246() + proof_record_247() + proof_record_248() + proof_record_249() + proof_record_250() + proof_record_251() + proof_record_252() + proof_record_253() + proof_record_254() + proof_record_255() + proof_record_256() + proof_record_257() + proof_record_258() + proof_record_259() + proof_record_260() + proof_record_261() + proof_record_262() + proof_record_263() + proof_record_264() + proof_record_265() + proof_record_266() + proof_record_267() + proof_record_268() + proof_record_269() + proof_record_270() + proof_record_271() + proof_record_272() + proof_record_273() + proof_record_274() + proof_record_275() + proof_record_276() + proof_record_277() + proof_record_278() + proof_record_279() + proof_record_280() + proof_record_281() + proof_record_282() + proof_record_283() + proof_record_284() + proof_record_285() + proof_record_286() + proof_record_287() + proof_record_288() + proof_record_289() + proof_record_290() + proof_record_291() + proof_record_292() + proof_record_293() + proof_record_294() + proof_record_295() + proof_record_296() + proof_record_297() + proof_record_298() + proof_record_299() + proof_record_300() + proof_record_301() + proof_record_302() + proof_record_303() + proof_record_304() + proof_record_305() + proof_record_306() + proof_record_307() + proof_record_308() + proof_record_309() + proof_record_310() + proof_record_311() + proof_record_312() + proof_record_313() + proof_record_314() + proof_record_315() + proof_record_316() + proof_record_317() + proof_record_318() + proof_record_319() + proof_record_320() + proof_record_321() + proof_record_322() + proof_record_323() + proof_record_324() + proof_record_325() + proof_record_326() + proof_record_327() + proof_record_328() + proof_record_329() + proof_record_330() + proof_record_331() + proof_record_332() + proof_record_333() + proof_record_334() + proof_record_335() + proof_record_336() + proof_record_337() + proof_record_338() + proof_record_339() + proof_record_340() + proof_record_341() + proof_record_342() + proof_record_343() + proof_record_344() + proof_record_345() + proof_record_346() + proof_record_347() + proof_record_348() + proof_record_349() + proof_record_350() + proof_record_351() + proof_record_352() + proof_record_353() + proof_record_354() + proof_record_355() + proof_record_356() + proof_record_357() + proof_record_358() + proof_record_359() + proof_record_360() + proof_record_361() + proof_record_362() + proof_record_363() + proof_record_364() + proof_record_365() + proof_record_366() + proof_record_367() + proof_record_368() + proof_record_369() + proof_record_370() + proof_record_371() + proof_record_372() + proof_record_373() + proof_record_374() + proof_record_375() + proof_record_376() + proof_record_377() + proof_record_378() + proof_record_379() + proof_record_380() + proof_record_381() + proof_record_382() + proof_record_383() + proof_record_384() + proof_record_385() + proof_record_386() + proof_record_387() + proof_record_388() + proof_record_389() + proof_record_390() + proof_record_391() + proof_record_392() + proof_record_393() + proof_record_394() + proof_record_395() + proof_record_396() + proof_record_397() + proof_record_398() + proof_record_399() + proof_record_400() + proof_record_401() + proof_record_402() + proof_record_403() + proof_record_404() + proof_record_405() + proof_record_406() + proof_record_407() + proof_record_408() + proof_record_409() + proof_record_410() + proof_record_411() + proof_record_412() + proof_record_413() + proof_record_414() + proof_record_415() + proof_record_416() + proof_record_417() + proof_record_418() + proof_record_419() + proof_record_420() + proof_record_421() + proof_record_422() + proof_record_423() + proof_record_424() + proof_record_425() + proof_record_426() + proof_record_427() + proof_record_428() + proof_record_429() + proof_record_430() + proof_record_431() + proof_record_432() + proof_record_433() + proof_record_434() + proof_record_435() + proof_record_436() + proof_record_437() + proof_record_438() + proof_record_439() + proof_record_440() + proof_record_441() + proof_record_442() + proof_record_443() + proof_record_444() + proof_record_445() + proof_record_446() + proof_record_447() + proof_record_448() + proof_record_449() + proof_record_450() + proof_record_451() + proof_record_452() + proof_record_453() + proof_record_454() + proof_record_455() + proof_record_456() + proof_record_457() + proof_record_458() + proof_record_459() + proof_record_460() + proof_record_461() + proof_record_462() + proof_record_463() + proof_record_464() + proof_record_465() + proof_record_466() + proof_record_467() + proof_record_468() + proof_record_469() + proof_record_470() + proof_record_471() + proof_record_472() + proof_record_473() + proof_record_474() + proof_record_475() + proof_record_476() + proof_record_477() + proof_record_478() + proof_record_479() + proof_record_480() + proof_record_481() + proof_record_482() + proof_record_483() + proof_record_484() + proof_record_485() + proof_record_486() + proof_record_487() + proof_record_488() + proof_record_489() + proof_record_490() + proof_record_491() + proof_record_492() + proof_record_493() + proof_record_494() + proof_record_495() + proof_record_496() + proof_record_497() + proof_record_498() + proof_record_499() + proof_record_500() + proof_record_501() + proof_record_502() + proof_record_503() + proof_record_504() + proof_record_505() + proof_record_506() + proof_record_507() + proof_record_508() + proof_record_509() + proof_record_510() + proof_record_511() + proof_record_512() + proof_record_513() + proof_record_514() + proof_record_515() + proof_record_516() + proof_record_517() + proof_record_518() + proof_record_519() + proof_record_520() + proof_record_521() + proof_record_522() + proof_record_523() + proof_record_524() + proof_record_525() + proof_record_526() + proof_record_527() + proof_record_528() + proof_record_529() + proof_record_530() + proof_record_531() + proof_record_532() + proof_record_533() + proof_record_534() + proof_record_535() + proof_record_536() + proof_record_537() + proof_record_538() + proof_record_539() + proof_record_540() + proof_record_541() + proof_record_542() + proof_record_543() + proof_record_544() + proof_record_545() + proof_record_546() + proof_record_547() + proof_record_548() + proof_record_549() + proof_record_550() + proof_record_551() + proof_record_552() + proof_record_553() + proof_record_554() + proof_record_555() + proof_record_556() + proof_record_557() + proof_record_558() + proof_record_559() + proof_record_560() + proof_record_561() + proof_record_562() + proof_record_563() + proof_record_564() + proof_record_565() + proof_record_566() + proof_record_567() + proof_record_568() + proof_record_569() + proof_record_570() + proof_record_571() + proof_record_572() + proof_record_573() + proof_record_574() + proof_record_575() + proof_record_576() + proof_record_577() + proof_record_578() + proof_record_579() + proof_record_580() + proof_record_581() + proof_record_582() + proof_record_583() + proof_record_584() + proof_record_585() + proof_record_586() + proof_record_587() + proof_record_588() + proof_record_589() + proof_record_590() + proof_record_591() + proof_record_592() + proof_record_593() + proof_record_594() + proof_record_595() + proof_record_596() + proof_record_597() + proof_record_598() + proof_record_599() + proof_record_600() + proof_record_601() + proof_record_602() + proof_record_603() + proof_record_604() + proof_record_605() + proof_record_606() + proof_record_607() + proof_record_608() + proof_record_609() + proof_record_610() + proof_record_611() + proof_record_612() + proof_record_613() + proof_record_614() + proof_record_615() + proof_record_616() + proof_record_617() + proof_record_618() + proof_record_619() + proof_record_620() + proof_record_621() + proof_record_622() + proof_record_623() + proof_record_624() + proof_record_625() + proof_record_626() + proof_record_627() + proof_record_628() + proof_record_629() + proof_record_630() + proof_record_631() + proof_record_632() + proof_record_633() + proof_record_634() + proof_record_635() + proof_record_636() + proof_record_637() + proof_record_638() + proof_record_639() + proof_record_640() + proof_record_641() + proof_record_642() + proof_record_643() + proof_record_644() + proof_record_645() + proof_record_646() + proof_record_647() + proof_record_648() + proof_record_649() + proof_record_650() + proof_record_651() + proof_record_652() + proof_record_653() + proof_record_654() + proof_record_655() + proof_record_656() + proof_record_657() + proof_record_658() + proof_record_659() + proof_record_660() + proof_record_661() + proof_record_662() + proof_record_663() + proof_record_664() + proof_record_665() + proof_record_666() + proof_record_667() + proof_record_668() + proof_record_669() + proof_record_670() + proof_record_671() + proof_record_672() + proof_record_673() + proof_record_674() + proof_record_675() + proof_record_676() + proof_record_677() + proof_record_678() + proof_record_679() + proof_record_680() + proof_record_681() + proof_record_682() + proof_record_683() + proof_record_684() + proof_record_685() + proof_record_686() + proof_record_687() + proof_record_688() + proof_record_689() + proof_record_690() + proof_record_691() + proof_record_692() + proof_record_693() + proof_record_694() + proof_record_695() + proof_record_696() + proof_record_697() + proof_record_698() + proof_record_699() + proof_record_700() + proof_record_701() + proof_record_702() + proof_record_703() + proof_record_704() + proof_record_705() + proof_record_706() + proof_record_707() + proof_record_708() + proof_record_709() + proof_record_710() + proof_record_711() + proof_record_712() + proof_record_713() + proof_record_714() + proof_record_715() + proof_record_716() + proof_record_717() + proof_record_718() + proof_record_719() + proof_record_720() + proof_record_721() + proof_record_722() + proof_record_723() + proof_record_724() + proof_record_725() + proof_record_726() + proof_record_727() + proof_record_728() + proof_record_729() + proof_record_730() + proof_record_731() + proof_record_732() + proof_record_733() + proof_record_734() + proof_record_735() + proof_record_736() + proof_record_737() + proof_record_738() + proof_record_739() + proof_record_740() + proof_record_741() + proof_record_742() + proof_record_743() + proof_record_744() + proof_record_745() + proof_record_746() + proof_record_747() + proof_record_748() + proof_record_749() + proof_record_750() + proof_record_751() + proof_record_752() + proof_record_753() + proof_record_754() + proof_record_755() + proof_record_756() + proof_record_757() + proof_record_758() + proof_record_759() + proof_record_760() + proof_record_761() + proof_record_762() + proof_record_763() + proof_record_764() + proof_record_765() + proof_record_766() + proof_record_767() + proof_record_768() + proof_record_769() + proof_record_770() + proof_record_771() + proof_record_772() + proof_record_773() + proof_record_774() + proof_record_775() + proof_record_776() + proof_record_777() + proof_record_778() + proof_record_779() + proof_record_780() + proof_record_781() + proof_record_782() + proof_record_783() + proof_record_784() + proof_record_785() + proof_record_786() + proof_record_787() + proof_record_788() + proof_record_789() + proof_record_790() + proof_record_791() + proof_record_792() + proof_record_793() + proof_record_794() + proof_record_795() + proof_record_796() + proof_record_797() + proof_record_798() + proof_record_799() + proof_record_800() + proof_record_801() + proof_record_802() + proof_record_803() + proof_record_804() + proof_record_805() + proof_record_806() + proof_record_807() + proof_record_808() + proof_record_809() + proof_record_810() + proof_record_811() + proof_record_812() + proof_record_813() + proof_record_814() + proof_record_815() + proof_record_816() + proof_record_817() + proof_record_818() + proof_record_819() + proof_record_820() + proof_record_821() + proof_record_822() + proof_record_823() + proof_record_824() + proof_record_825() + proof_record_826() + proof_record_827() + proof_record_828() + proof_record_829() + proof_record_830() + proof_record_831() + proof_record_832() + proof_record_833() + proof_record_834() + proof_record_835() + proof_record_836() + proof_record_837() + proof_record_838() + proof_record_839() + proof_record_840() + proof_record_841() + proof_record_842() + proof_record_843() + proof_record_844() + proof_record_845() + proof_record_846() + proof_record_847() + proof_record_848() + proof_record_849() + proof_record_850() + proof_record_851() + proof_record_852() + proof_record_853() + proof_record_854() + proof_record_855() + proof_record_856() + proof_record_857() + proof_record_858() + proof_record_859() + proof_record_860() + proof_record_861() + proof_record_862() + proof_record_863() + proof_record_864() + proof_record_865() + proof_record_866() + proof_record_867() + proof_record_868() + proof_record_869() + proof_record_870() + proof_record_871() + proof_record_872() + proof_record_873() + proof_record_874() + proof_record_875() + proof_record_876() + proof_record_877() + proof_record_878() + proof_record_879() + proof_record_880() + proof_record_881() + proof_record_882() + proof_record_883() + proof_record_884() + proof_record_885() + proof_record_886() + proof_record_887() + proof_record_888() + proof_record_889() + proof_record_890() + proof_record_891() + proof_record_892() + proof_record_893() + proof_record_894() + proof_record_895() + proof_record_896() + proof_record_897() + proof_record_898() + proof_record_899() + proof_record_900() + proof_record_901() + proof_record_902() + proof_record_903() + proof_record_904() + proof_record_905() + proof_record_906() + proof_record_907() + proof_record_908() + proof_record_909() + proof_record_910() + proof_record_911() + proof_record_912() + proof_record_913() + proof_record_914() + proof_record_915() + proof_record_916() + proof_record_917() + proof_record_918() + proof_record_919() + proof_record_920() + proof_record_921() + proof_record_922() + proof_record_923() + proof_record_924() + proof_record_925() + proof_record_926() + proof_record_927() + proof_record_928() + proof_record_929() + proof_record_930() + proof_record_931() + proof_record_932() + proof_record_933() + proof_record_934() + proof_record_935() + proof_record_936() + proof_record_937() + proof_record_938() + proof_record_939() + proof_record_940() + proof_record_941() + proof_record_942() + proof_record_943() + proof_record_944() + proof_record_945() + proof_record_946() + proof_record_947() + proof_record_948() + proof_record_949() + proof_record_950() + proof_record_951() + proof_record_952() + proof_record_953() + proof_record_954() + proof_record_955() + proof_record_956() + proof_record_957() + proof_record_958() + proof_record_959() + proof_record_960() + proof_record_961() + proof_record_962() + proof_record_963() + proof_record_964() + proof_record_965() + proof_record_966() + proof_record_967() + proof_record_968() + proof_record_969() + proof_record_970() + proof_record_971() + proof_record_972() + proof_record_973() + proof_record_974() + proof_record_975() + proof_record_976() + proof_record_977() + proof_record_978() + proof_record_979() + proof_record_980() + proof_record_981() + proof_record_982() + proof_record_983() + proof_record_984() + proof_record_985() + proof_record_986() + proof_record_987() + proof_record_988() + proof_record_989() + proof_record_990() + proof_record_991() + proof_record_992() + proof_record_993() + proof_record_994() + proof_record_995() + proof_record_996() + proof_record_997() + proof_record_998() + proof_record_999() + proof_record_1000() + proof_record_1001() + proof_record_1002() + proof_record_1003() + proof_record_1004() + proof_record_1005() + proof_record_1006() + proof_record_1007() + proof_record_1008() + proof_record_1009() + proof_record_1010() + proof_record_1011() + proof_record_1012() + proof_record_1013() + proof_record_1014() + proof_record_1015() + proof_record_1016() + proof_record_1017() + proof_record_1018() + proof_record_1019() + proof_record_1020() + proof_record_1021() + proof_record_1022() + proof_record_1023() + proof_record_1024() + proof_record_1025() + proof_record_1026() + proof_record_1027() + proof_record_1028() + proof_record_1029() + proof_record_1030() + proof_record_1031() + proof_record_1032() + proof_record_1033() + proof_record_1034() + proof_record_1035() + proof_record_1036() + proof_record_1037() + proof_record_1038() + proof_record_1039() + proof_record_1040() + proof_record_1041() + proof_record_1042() + proof_record_1043() + proof_record_1044() + proof_record_1045() + proof_record_1046() + proof_record_1047() + proof_record_1048() + proof_record_1049() + proof_record_1050() + proof_record_1051() + proof_record_1052() + proof_record_1053() + proof_record_1054() + proof_record_1055() + proof_record_1056() + proof_record_1057() + proof_record_1058() + proof_record_1059() + proof_record_1060() + proof_record_1061() + proof_record_1062() + proof_record_1063() + proof_record_1064() + proof_record_1065() + proof_record_1066() + proof_record_1067() + proof_record_1068() + proof_record_1069() + proof_record_1070() + proof_record_1071() + proof_record_1072() + proof_record_1073() + proof_record_1074() + proof_record_1075() + proof_record_1076() + proof_record_1077() + proof_record_1078() + proof_record_1079() + proof_record_1080() + proof_record_1081() + proof_record_1082() + proof_record_1083() + proof_record_1084() + proof_record_1085() + proof_record_1086() + proof_record_1087() + proof_record_1088() + proof_record_1089() + proof_record_1090() + proof_record_1091() + proof_record_1092() + proof_record_1093() + proof_record_1094() + proof_record_1095() + proof_record_1096() + proof_record_1097() + proof_record_1098() + proof_record_1099() + proof_record_1100() + proof_record_1101() + proof_record_1102() + proof_record_1103() + proof_record_1104() + proof_record_1105() + proof_record_1106() + proof_record_1107() + proof_record_1108() + proof_record_1109() + proof_record_1110() + proof_record_1111() + proof_record_1112() + proof_record_1113() + proof_record_1114() + proof_record_1115() + proof_record_1116() + proof_record_1117() + proof_record_1118() + proof_record_1119() + proof_record_1120() + proof_record_1121() + proof_record_1122() + proof_record_1123() + proof_record_1124() + proof_record_1125() + proof_record_1126() + proof_record_1127() + proof_record_1128() + proof_record_1129() + proof_record_1130() + proof_record_1131() + proof_record_1132() + proof_record_1133() + proof_record_1134() + proof_record_1135() + proof_record_1136() + proof_record_1137() + proof_record_1138() + proof_record_1139() + proof_record_1140() + proof_record_1141() + proof_record_1142() + proof_record_1143() + proof_record_1144() + proof_record_1145() + proof_record_1146() + proof_record_1147() + proof_record_1148() + proof_record_1149() + proof_record_1150() + proof_record_1151() + proof_record_1152() + proof_record_1153() + proof_record_1154() + proof_record_1155() + proof_record_1156() + proof_record_1157() + proof_record_1158() + proof_record_1159() + proof_record_1160() + proof_record_1161() + proof_record_1162() + proof_record_1163() + proof_record_1164() + proof_record_1165() + proof_record_1166() + proof_record_1167() + proof_record_1168() + proof_record_1169() + proof_record_1170() + proof_record_1171() + proof_record_1172() + proof_record_1173() + proof_record_1174() + proof_record_1175() + proof_record_1176() + proof_record_1177() + proof_record_1178() + proof_record_1179() + proof_record_1180() + proof_record_1181() + proof_record_1182() + proof_record_1183() + proof_record_1184() + proof_record_1185() + proof_record_1186() + proof_record_1187() + proof_record_1188() + proof_record_1189() + proof_record_1190() + proof_record_1191() + proof_record_1192() + proof_record_1193() + proof_record_1194() + proof_record_1195() + proof_record_1196() + proof_record_1197() + proof_record_1198() + proof_record_1199() + proof_record_1200() + proof_record_1201() + proof_record_1202() + proof_record_1203() + proof_record_1204() + proof_record_1205() + proof_record_1206() + proof_record_1207() + proof_record_1208() + proof_record_1209() + proof_record_1210() + proof_record_1211() + proof_record_1212() + proof_record_1213() + proof_record_1214() + proof_record_1215() + proof_record_1216() + proof_record_1217() + proof_record_1218() + proof_record_1219() + proof_record_1220() + proof_record_1221() + proof_record_1222() + proof_record_1223() + proof_record_1224() + proof_record_1225() + proof_record_1226() + proof_record_1227() + proof_record_1228() + proof_record_1229() + proof_record_1230() + proof_record_1231() + proof_record_1232() + proof_record_1233() + proof_record_1234() + proof_record_1235() + proof_record_1236() + proof_record_1237() + proof_record_1238() + proof_record_1239() + proof_record_1240() + proof_record_1241() + proof_record_1242() + proof_record_1243() + proof_record_1244() + proof_record_1245() + proof_record_1246() + proof_record_1247() + proof_record_1248() + proof_record_1249() + proof_record_1250() + proof_record_1251() + proof_record_1252() + proof_record_1253() + proof_record_1254() + proof_record_1255() + proof_record_1256() + proof_record_1257() + proof_record_1258() + proof_record_1259() + proof_record_1260() + proof_record_1261() + proof_record_1262() + proof_record_1263() + proof_record_1264() + proof_record_1265() + proof_record_1266() + proof_record_1267() + proof_record_1268() + proof_record_1269() + proof_record_1270() + proof_record_1271() + proof_record_1272() + proof_record_1273() + proof_record_1274() + proof_record_1275() + proof_record_1276() + proof_record_1277() + proof_record_1278() + proof_record_1279() + proof_record_1280() + proof_record_1281() + proof_record_1282() + proof_record_1283() + proof_record_1284() + proof_record_1285() + proof_record_1286() + proof_record_1287() + proof_record_1288() + proof_record_1289() + proof_record_1290() + proof_record_1291() + proof_record_1292() + proof_record_1293() + proof_record_1294() + proof_record_1295() + proof_record_1296() + proof_record_1297() + proof_record_1298() + proof_record_1299() + proof_record_1300() + proof_record_1301() + proof_record_1302() + proof_record_1303() + proof_record_1304() + proof_record_1305() + proof_record_1306() + proof_record_1307() + proof_record_1308() + proof_record_1309() + proof_record_1310() + proof_record_1311() + proof_record_1312() + proof_record_1313() + proof_record_1314() + proof_record_1315() + proof_record_1316() + proof_record_1317() + proof_record_1318() + proof_record_1319() + proof_record_1320() + proof_record_1321() + proof_record_1322() + proof_record_1323() + proof_record_1324() + proof_record_1325() + proof_record_1326() + proof_record_1327() + proof_record_1328() + proof_record_1329() + proof_record_1330() + proof_record_1331() + proof_record_1332() + proof_record_1333() + proof_record_1334() + proof_record_1335() + proof_record_1336() + proof_record_1337() + proof_record_1338() + proof_record_1339() + proof_record_1340() + proof_record_1341() + proof_record_1342() + proof_record_1343() + proof_record_1344() + proof_record_1345() + proof_record_1346() + proof_record_1347() + proof_record_1348() + proof_record_1349() + proof_record_1350() + proof_record_1351() + proof_record_1352() + proof_record_1353() + proof_record_1354() + proof_record_1355() + proof_record_1356() + proof_record_1357() + proof_record_1358() + proof_record_1359() + proof_record_1360() + proof_record_1361() + proof_record_1362() + proof_record_1363() + proof_record_1364() + proof_record_1365() + proof_record_1366() + proof_record_1367() + proof_record_1368() + proof_record_1369() + proof_record_1370() + proof_record_1371() + proof_record_1372() + proof_record_1373() + proof_record_1374() + proof_record_1375() + proof_record_1376() + proof_record_1377() + proof_record_1378() + proof_record_1379() + proof_record_1380() + proof_record_1381() + proof_record_1382() + proof_record_1383() + proof_record_1384() + proof_record_1385() + proof_record_1386() + proof_record_1387() + proof_record_1388() + proof_record_1389() + proof_record_1390() + proof_record_1391() + proof_record_1392() + proof_record_1393() + proof_record_1394() + proof_record_1395() + proof_record_1396() + proof_record_1397() + proof_record_1398() + proof_record_1399() + proof_record_1400() + proof_record_1401() + proof_record_1402() + proof_record_1403() + proof_record_1404() + proof_record_1405() + proof_record_1406() + proof_record_1407() + proof_record_1408() + proof_record_1409() + proof_record_1410() + proof_record_1411() + proof_record_1412() + proof_record_1413() + proof_record_1414() + proof_record_1415() + proof_record_1416() + proof_record_1417() + proof_record_1418() + proof_record_1419() + proof_record_1420() + proof_record_1421() + proof_record_1422() + proof_record_1423() + proof_record_1424() + proof_record_1425() + proof_record_1426() + proof_record_1427() + proof_record_1428() + proof_record_1429() + proof_record_1430() + proof_record_1431() + proof_record_1432() + proof_record_1433() + proof_record_1434() + proof_record_1435() + proof_record_1436() + proof_record_1437() + proof_record_1438() + proof_record_1439() + proof_record_1440() + proof_record_1441() + proof_record_1442() + proof_record_1443() + proof_record_1444() + proof_record_1445() + proof_record_1446() + proof_record_1447() + proof_record_1448() + proof_record_1449() + proof_record_1450() + proof_record_1451() + proof_record_1452() + proof_record_1453() + proof_record_1454() + proof_record_1455() + proof_record_1456() + proof_record_1457() + proof_record_1458() + proof_record_1459() + proof_record_1460() + proof_record_1461() + proof_record_1462() + proof_record_1463() + proof_record_1464() + proof_record_1465() + proof_record_1466() + proof_record_1467() + proof_record_1468() + proof_record_1469() + proof_record_1470() + proof_record_1471() + proof_record_1472() + proof_record_1473() + proof_record_1474() + proof_record_1475() + proof_record_1476() + proof_record_1477() + proof_record_1478() + proof_record_1479() + proof_record_1480() + proof_record_1481() + proof_record_1482() + proof_record_1483() + proof_record_1484() + proof_record_1485() + proof_record_1486() + proof_record_1487() + proof_record_1488() + proof_record_1489() + proof_record_1490() + proof_record_1491() + proof_record_1492() + proof_record_1493() + proof_record_1494() + proof_record_1495() + proof_record_1496() + proof_record_1497() + proof_record_1498() + proof_record_1499();
    let message = ProofMessage {
        ok: true,
        source: "cinder-proof",
        checksum,
    };

    println!("{}", serde_json::to_string(&message).unwrap());
}
