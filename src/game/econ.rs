//Individual - General
//Is - Demographics
pub struct Age;
pub enum Gender {
    Male,
    Female
}
pub struct Religion;
pub struct Name(Vec<NameItem>);
pub enum NameItem {
    Prefix(PrefixName),
    First(&'static str),
    Middle(&'static str),
    Maiden(&'static str),
    Last(&'static str),
    Suffix(SuffixName),
}
pub enum PrefixName {
    Mr,
    Mrs,
    Ms,
    Dr,
}
pub enum SuffixName {
    Generation(i32),
    Jr,
    Sr,
}

//Has Single
pub struct Hometown;

//Has Containers
pub struct Body;
pub struct Clothes;
pub struct Family;
pub struct Place;
pub struct Estate;
pub struct Education;
pub struct Mind;