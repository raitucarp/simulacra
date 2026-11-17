//! What kind of being an agent is (flat ontology: a person, a cat, a tin can and a river are
//! all equally objects) and the concrete form it takes. Every form belongs to one kind.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// The ontological kind of an agent's body.
    pub enum BodyKind in "body_kind" {
        #[default]
        Human("human", "Human Being"),
        Creature("creature", "Creature (Animal)"),
        Plant("plant", "Plant or Fungus"),
        Object("object", "Everyday Object"),
        Element("element", "Element or Substance"),
        Machine("machine", "Machine or Device"),
        Place("place", "Place or Landscape"),
        Abstraction("abstraction", "Abstraction or Concept"),
    }
}

trait_enum! {
    /// The concrete form an agent takes (see [`BodyForm::kind`]).
    pub enum BodyForm in "body_form" {
        #[default]
        Person("person", "Person"),
        TinCan("tin_can", "Tin Can"), Book("book", "Book"), Chair("chair", "Chair"), Lamp("lamp", "Lamp"),
        Clock("clock", "Clock"), Mirror("mirror", "Mirror"), Key("key", "Key"), Teacup("teacup", "Teacup"),
        OakTree("oak_tree", "Oak Tree"), Cactus("cactus", "Cactus"), Moss("moss", "Moss"), Mushroom("mushroom", "Mushroom"),
        Sunflower("sunflower", "Sunflower"), Vine("vine", "Vine"), Bamboo("bamboo", "Bamboo"), Seed("seed", "Seed"),
        Water("water", "Water"), Fire("fire", "Fire"), Wind("wind", "Wind"), Stone("stone", "Stone"),
        Light("light", "Light"), Ice("ice", "Ice"), Cloud("cloud", "Cloud"), Sand("sand", "Sand"),
        Cat("cat", "Cat"), Crow("crow", "Crow"), Octopus("octopus", "Octopus"), Wolf("wolf", "Wolf"),
        Bee("bee", "Bee"), Whale("whale", "Whale"), Snail("snail", "Snail"), Owl("owl", "Owl"),
        Robot("robot", "Robot"), Satellite("satellite", "Satellite"), Server("server", "Server"),
        Typewriter("typewriter", "Typewriter"), Drone("drone", "Drone"), Radio("radio", "Radio"),
        Calculator("calculator", "Calculator"), Train("train", "Train"),
        City("city", "City"), Island("island", "Island"), Library("library", "Library"), Cathedral("cathedral", "Cathedral"),
        Desert("desert", "Desert"), Forest("forest", "Forest"), Harbor("harbor", "Harbor"), Ruin("ruin", "Ruin"),
        Time("time", "Time"), Justice("justice", "Justice"), Number("number", "Number"), Silence("silence", "Silence"),
        Memory("memory", "Memory"), Chance("chance", "Chance"), Love("love", "Love"), Entropy("entropy", "Entropy"),
    }
}

impl BodyForm {
    /// The kind this form belongs to.
    pub const fn kind(self) -> BodyKind {
        use BodyForm::*;
        match self {
            Person => BodyKind::Human,
            TinCan | Book | Chair | Lamp | Clock | Mirror | Key | Teacup => BodyKind::Object,
            OakTree | Cactus | Moss | Mushroom | Sunflower | Vine | Bamboo | Seed => BodyKind::Plant,
            Water | Fire | Wind | Stone | Light | Ice | Cloud | Sand => BodyKind::Element,
            Cat | Crow | Octopus | Wolf | Bee | Whale | Snail | Owl => BodyKind::Creature,
            Robot | Satellite | Server | Typewriter | Drone | Radio | Calculator | Train => BodyKind::Machine,
            City | Island | Library | Cathedral | Desert | Forest | Harbor | Ruin => BodyKind::Place,
            Time | Justice | Number | Silence | Memory | Chance | Love | Entropy => BodyKind::Abstraction,
        }
    }
}

impl BodyKind {
    /// Every form of this kind, in display order.
    pub fn forms(self) -> Vec<BodyForm> {
        BodyForm::ALL.into_iter().filter(|form| form.kind() == self).collect()
    }

    /// The first form of this kind (what picking the kind starts from).
    pub fn default_form(self) -> BodyForm {
        self.forms().first().copied().unwrap_or_default()
    }
}
