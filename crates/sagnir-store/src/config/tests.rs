use super::*;
extern crate std;
use std::format;

#[test]
fn default_config_is_strict_standard_and_round_trips() {
    let config = RealmConfig::default();
    let mut output = [0_u8; CONFIG_FILE_MAX];
    let encoded = concat!(
        "format = 1\n",
        "profile = \"standard\"\n",
        "\n",
        "[verification]\n",
        "mode = \"lazy-cone\"\n",
        "memory_budget = \"512MiB\"\n",
    );

    assert_eq!(config.profile, Profile::Standard);
    assert_eq!(config.verification.mode, VerificationMode::LazyCone);
    assert_eq!(write_config_toml(config, &mut output), Ok(encoded.len()));
    assert_eq!(&output[..encoded.len()], encoded.as_bytes());
    assert_eq!(parse_config_toml(encoded), Ok(config));
    assert!(encoded.contains("memory_budget = \"512MiB\""));
}

#[test]
fn parser_accepts_every_profile_and_mode() {
    for profile in ["standard", "solo", "team", "regulated"] {
        for mode in ["bounded-batch", "lazy-cone", "full-world"] {
            let input =
                format!("format = 1\nprofile = \"{profile}\"\n[verification]\nmode = \"{mode}\"\n");
            assert!(parse_config_toml(&input).is_ok(), "{profile} {mode}");
        }
    }
}

#[test]
fn parser_accepts_explicit_verification_budgets() {
    let input = concat!(
        "format = 1\n",
        "profile = \"regulated\"\n",
        "[verification]\n",
        "mode = \"full-world\"\n",
        "memory_budget = \"96GiB\"\n",
        "parallelism = 32\n",
        "max_entries = 500000\n",
        "max_refs = 1000000\n",
    );
    assert_eq!(
        parse_config_toml(input),
        Ok(RealmConfig {
            profile: Profile::Regulated,
            verification: VerificationConfig {
                mode: VerificationMode::FullWorld,
                memory_budget: Some(MemoryBudget(96 * 1_024_u64.pow(3))),
                parallelism: Some(32),
                max_entries: Some(500_000),
                max_refs: Some(1_000_000),
            },
        })
    );
}

#[test]
fn parser_rejects_unknown_profiles_modes_and_fields() {
    for input in [
        "format = 1\nprofile = \"open\"\n[verification]\nmode = \"lazy-cone\"\n",
        "format = 1\nprofile = \"standard\"\n[verification]\nmode = \"unbounded\"\n",
        "format = 1\nprofile = \"standard\"\nunknown = 1\n[verification]\nmode = \"lazy-cone\"\n",
    ] {
        assert!(parse_config_toml(input).is_err());
    }
}

#[test]
fn parser_rejects_duplicate_and_missing_fields() {
    let duplicate = concat!(
        "format = 1\n",
        "profile = \"standard\"\n",
        "profile = \"solo\"\n",
        "[verification]\n",
        "mode = \"lazy-cone\"\n",
    );
    assert_eq!(
        parse_config_toml(duplicate),
        Err(StoreMetadataError::DuplicateField)
    );
    assert_eq!(
        parse_config_toml("format = 1\nprofile = \"standard\"\n"),
        Err(StoreMetadataError::MissingField)
    );

    let duplicate_budget = concat!(
        "format = 1\n",
        "profile = \"standard\"\n",
        "[verification]\n",
        "mode = \"lazy-cone\"\n",
        "memory_budget = \"512MiB\"\n",
        "memory_budget = \"1GiB\"\n",
    );
    assert_eq!(
        parse_config_toml(duplicate_budget),
        Err(StoreMetadataError::DuplicateField)
    );
}

#[test]
fn parser_rejects_dangerous_or_out_of_range_budgets() {
    let template = |line: &str| {
        format!(
            "format = 1\nprofile = \"standard\"\n[verification]\nmode = \"lazy-cone\"\n{line}\n"
        )
    };
    for line in [
        "memory_budget = \"0GiB\"",
        "memory_budget = \"999999999999999999999GiB\"",
        "memory_budget = \"1GB\"",
        "parallelism = 0",
        "parallelism = 257",
        "max_entries = 0",
        "max_refs = 4294967296",
    ] {
        assert!(parse_config_toml(&template(line)).is_err(), "{line}");
    }
}

#[test]
fn memory_budget_accepts_exact_admitted_endpoints() {
    let parse = |budget: &str| {
        parse_config_toml(&format!(
            "format = 1\nprofile = \"standard\"\n[verification]\nmode = \"full-world\"\nmemory_budget = \"{budget}\"\n"
        ))
    };

    assert!(parse("16MiB").is_ok());
    assert!(parse("1TiB").is_ok());
    assert!(parse("1024GiB").is_ok());
    assert!(parse("1025GiB").is_err());
}

#[test]
fn writer_rejects_small_buffers_and_invalid_constructed_values() {
    let config = RealmConfig::default();
    assert_eq!(
        write_config_toml(config, &mut [0_u8; 8]),
        Err(StoreMetadataError::BufferTooSmall)
    );

    let invalid = RealmConfig {
        verification: VerificationConfig {
            parallelism: Some(0),
            ..config.verification
        },
        ..config
    };
    assert_eq!(
        write_config_toml(invalid, &mut [0_u8; CONFIG_FILE_MAX]),
        Err(StoreMetadataError::ValueOutOfRange)
    );
}
