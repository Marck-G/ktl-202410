use super::entities::Person;

pub trait Mapper<I, D> {
    fn to_domain(infra: I, metadata: D) -> Person;
    fn to_infrastructure(domain: Person) -> (I, D);
}
