// rustfmt-match_arm_align_threshold: 100
// Align match arms

fn main() {
    match lorem {
        Lorem::Ipsum     => (),
        Lorem::DolorSitAmetConsecteturAdipiscingElitSedDo     => (),
        Lorem::Eiusmod     => {
            lorem();
            ipsum();
        }
        Lorem::Donec | Lorem::Hendrerit | Lorem::Tempor | Lorem::Tellus | Lorem::Proin |
        Lorem::Quam | Lorem::Nisl    => (),
        Lorem::Donec => (),
        Lorem::Hendrerit |
        Lorem::TemporTellusProinQuamNislTinciduntEtMattisEgetConvallisNecPurusCumSociis     => (),
        Lorem::Natoque if lorem()     => (),
    }
}
