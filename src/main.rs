use older::Version as SpookyVersion;
use semver::Version;

fn main() {
    let version_1 = bincode::serialize(&Version::parse("1.3.0").unwrap()).unwrap();
    let version_2 = bincode::serialize(&SpookyVersion::parse("1.3.0").unwrap()).unwrap();
    assert_eq!(version_1, version_2);
}
