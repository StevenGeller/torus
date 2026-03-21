#![allow(unreachable_patterns)]
/// Extended word→semantic prime decompositions organized by semantic field.
/// Each field has a prime template, and words are grouped by field.
/// This extends the ~150 hand-curated entries in language.rs to cover
/// the most common English words (~500+ additional entries).

use crate::language::SemanticPrime;

fn p(prime: &'static str, domain: &'static str) -> SemanticPrime {
    SemanticPrime { prime, domain }
}

pub fn lookup(word: &str) -> Option<Vec<SemanticPrime>> {
    Some(match word {
        // ═══ Animals ═══
        "dog" | "cat" | "horse" | "cow" | "pig" | "sheep" | "goat"
        | "deer" | "rabbit" | "fox" | "wolf" | "monkey" | "ape"
        | "rat" | "mouse" | "squirrel" | "raccoon" | "skunk"
        | "donkey" | "mule" | "ox" | "bull" | "lamb" | "pony"
        | "hare" | "hedgehog" | "badger" | "otter" | "beaver"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action")],

        "elephant" | "whale" | "dinosaur" | "dragon" | "giant"
        | "bear" | "lion" | "tiger" | "gorilla" | "rhino"
        | "hippo" | "moose" | "buffalo" | "camel" | "giraffe"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action"), p("BIG", "descriptor")],

        "ant" | "bee" | "spider" | "worm" | "beetle" | "mosquito"
        | "flea" | "moth" | "caterpillar" | "snail" | "slug"
        | "cricket" | "grasshopper" | "ladybug" | "firefly"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action"), p("SMALL", "descriptor")],

        "bird" | "eagle" | "hawk" | "owl" | "crow" | "raven"
        | "sparrow" | "dove" | "pigeon" | "parrot" | "swan"
        | "falcon" | "vulture" | "heron" | "stork" | "robin"
        | "butterfly" | "dragonfly" | "hummingbird" | "bat"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action"), p("ABOVE", "space")],

        "fish" | "shark" | "dolphin" | "octopus" | "crab" | "lobster"
        | "seal" | "penguin" | "jellyfish" | "squid" | "clam"
        | "oyster" | "shrimp" | "eel" | "salmon" | "trout" | "tuna"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action"), p("BELOW", "space")],

        "snake" | "lizard" | "frog" | "toad" | "turtle" | "crocodile"
        | "alligator" | "gecko" | "iguana" | "chameleon" | "salamander"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("MOVE", "action"), p("BELOW", "space")],

        // ═══ Plants ═══
        "plant" | "grass" | "vine" | "moss" | "bush" | "shrub"
        | "weed" | "herb" | "reed" | "fern" | "ivy" | "cactus"
        | "seaweed" | "algae" | "bamboo" | "crop" | "grain"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("WHERE/PLACE", "space")],

        "tree" | "oak" | "pine" | "maple" | "willow" | "cedar"
        | "palm" | "birch" | "elm" | "ash" | "cypress" | "redwood"
        | "sequoia" | "spruce" | "beech" | "walnut" | "cherry"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("BIG", "descriptor"), p("WHERE/PLACE", "space")],

        "flower" | "rose" | "lily" | "daisy" | "tulip" | "orchid"
        | "sunflower" | "violet" | "lotus" | "jasmine" | "lavender"
        | "iris" | "poppy" | "carnation" | "blossom" | "petal" | "bloom"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("SEE", "mental"), p("GOOD", "evaluator")],

        "seed" | "root" | "branch" | "leaf" | "bark" | "thorn"
        | "stem" | "bud" | "fruit" | "berry" | "acorn" | "pine cone"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("SOMETHING", "substantive")],

        // ═══ Food & Drink ═══
        "food" | "bread" | "meat" | "rice" | "wheat" | "corn"
        | "cheese" | "egg" | "butter" | "salt" | "sugar" | "pepper"
        | "cake" | "pie" | "soup" | "stew" | "meal" | "feast"
        | "flour" | "dough" | "pastry" | "cereal" | "pasta" | "noodle"
        | "sausage" | "bacon" | "ham" | "steak" | "chicken" | "pork"
        | "beef" | "lamb" | "sauce" | "spice" | "curry" | "chocolate"
        | "candy" | "cookie" | "biscuit" | "jam" | "jelly"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "apple" | "orange" | "grape" | "banana" | "peach" | "pear"
        | "plum" | "lemon" | "lime" | "melon" | "mango" | "coconut"
        | "pineapple" | "strawberry" | "raspberry" | "blueberry"
        | "fig" | "date" | "olive" | "cherry" | "pomegranate"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("GOOD", "evaluator")],

        "potato" | "onion" | "garlic" | "carrot" | "tomato"
        | "cabbage" | "lettuce" | "spinach" | "celery" | "cucumber"
        | "pepper" | "bean" | "pea" | "lentil" | "turnip" | "beet"
        | "radish" | "mushroom" | "nut" | "almond" | "peanut"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("BODY", "substantive")],

        "wine" | "beer" | "tea" | "coffee" | "juice" | "milk"
        | "cream" | "broth" | "syrup" | "vinegar" | "ale" | "mead"
        | "cider" | "rum" | "whiskey" | "vodka" | "brandy"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("MOVE", "action")],

        "honey" => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("GOOD", "evaluator"), p("MUCH/MANY", "quantifier")],

        // ═══ Body Parts ═══
        "arm" | "leg" | "foot" | "finger" | "toe" | "knee"
        | "shoulder" | "chest" | "neck" | "throat" | "hip"
        | "elbow" | "wrist" | "ankle" | "rib" | "spine" | "jaw"
        | "skull" | "pelvis" | "thigh" | "calf" | "shin" | "palm"
        | "fist" | "knuckle" | "heel"
        => vec![p("BODY", "substantive"), p("SOMETHING", "substantive")],

        "brain" | "nerve" | "muscle" | "stomach" | "lung" | "liver"
        | "kidney" | "intestine" | "vein" | "artery" | "organ"
        | "gland" | "marrow" | "tendon" | "ligament" | "cartilage"
        => vec![p("BODY", "substantive"), p("INSIDE", "space"), p("LIVE", "life")],

        "lip" | "tongue" | "tooth" | "teeth" | "ear" | "nose"
        | "hair" | "beard" | "eyebrow" | "eyelash" | "cheek"
        | "chin" | "forehead" | "scalp" | "nostril" | "nail"
        => vec![p("BODY", "substantive"), p("SEE", "mental"), p("SOMETHING", "substantive")],

        "wing" | "tail" | "horn" | "claw" | "paw" | "hoof"
        | "beak" | "feather" | "fur" | "scale" | "mane" | "tusk"
        | "fang" | "fin" | "gill" | "antenna" | "shell" | "web"
        => vec![p("BODY", "substantive"), p("LIVE", "life"), p("SOMETHING", "substantive")],

        "back" => vec![p("BODY", "substantive"), p("SOMETHING", "substantive"), p("BEHIND", "space")],
        "wound" | "scar" | "bruise" | "blister" | "rash" => vec![p("BODY", "substantive"), p("BAD", "evaluator")],

        // ═══ Materials ═══
        "wood" | "timber" | "lumber" | "plank" | "board" | "log"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("BIG", "descriptor")],

        "metal" | "iron" | "steel" | "copper" | "bronze" | "tin"
        | "lead" | "aluminum" | "zinc" | "titanium" | "chrome"
        | "alloy" | "ore" | "ingot"
        => vec![p("SOMETHING", "substantive"), p("BIG", "descriptor"), p("CAN", "logical")],

        "gold" | "silver" | "platinum" | "gem" | "jewel" | "pearl"
        | "diamond" | "ruby" | "emerald" | "sapphire" | "crystal"
        | "amber" | "jade" | "opal" | "amethyst" | "treasure"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("GOOD", "evaluator"), p("MUCH/MANY", "quantifier")],

        "glass" | "clay" | "sand" | "dust" | "dirt" | "mud"
        | "gravel" | "pebble" | "chalk" | "cement" | "concrete"
        | "brick" | "tile" | "marble" | "granite" | "slate"
        | "limestone" | "sandstone" | "ore" | "mineral"
        => vec![p("SOMETHING", "substantive"), p("BELOW", "space")],

        "silk" | "wool" | "cotton" | "linen" | "leather" | "fabric"
        | "thread" | "yarn" | "string" | "twine" | "tape" | "ribbon"
        | "cloth" | "textile" | "velvet" | "satin" | "canvas" | "burlap"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("DO", "action")],

        "paper" | "parchment" | "scroll" | "cardboard"
        => vec![p("SOMETHING", "substantive"), p("WORDS", "speech"), p("SEE", "mental")],

        "wax" | "oil" | "tar" | "resin" | "glue" | "paste"
        | "ink" | "dye" | "paint" | "pigment"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("DO", "action")],

        "coal" | "ash" | "ember" | "charcoal" | "soot" | "smoke"
        | "steam" | "vapor" | "gas" | "fume"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("ABOVE", "space")],

        // ═══ Buildings & Places ═══
        "house" | "room" | "hall" | "chamber" | "cabin" | "cottage"
        | "hut" | "shed" | "tent" | "shelter" | "dwelling" | "apartment"
        | "mansion" | "villa" | "estate" | "lodge" | "inn" | "tavern"
        | "hotel" | "hostel" | "dormitory" | "bunker" | "den" | "lair"
        => vec![p("WHERE/PLACE", "space"), p("INSIDE", "space"), p("LIVE", "life")],

        "castle" | "palace" | "fortress" | "citadel" | "keep"
        | "stronghold" | "tower" | "bastion" | "garrison"
        => vec![p("WHERE/PLACE", "space"), p("BIG", "descriptor"), p("ABOVE", "space"), p("CAN", "logical")],

        "temple" | "church" | "cathedral" | "chapel" | "mosque"
        | "synagogue" | "shrine" | "monastery" | "convent" | "abbey"
        | "sanctuary" | "altar"
        => vec![p("WHERE/PLACE", "space"), p("ABOVE", "space"), p("FEEL", "mental")],

        "school" | "library" | "university" | "academy" | "college"
        | "classroom" | "laboratory" | "studio" | "workshop"
        => vec![p("WHERE/PLACE", "space"), p("KNOW", "mental"), p("THINK", "mental")],

        "hospital" | "clinic" | "pharmacy"
        => vec![p("WHERE/PLACE", "space"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "prison" | "jail" | "dungeon" | "cell" | "cage"
        => vec![p("WHERE/PLACE", "space"), p("NOT", "logical"), p("CAN", "logical")],

        "market" | "store" | "shop" | "bazaar" | "mall"
        | "warehouse" | "depot" | "dock" | "port" | "harbor"
        => vec![p("WHERE/PLACE", "space"), p("HAVE", "existence"), p("DO", "action")],

        "factory" | "mill" | "forge" | "foundry" | "refinery"
        | "plant" | "mine" | "quarry"
        => vec![p("WHERE/PLACE", "space"), p("DO", "action"), p("SOMETHING", "substantive")],

        "farm" | "barn" | "stable" | "ranch" | "orchard" | "vineyard"
        | "garden" | "greenhouse" | "nursery" | "pasture" | "pen"
        => vec![p("WHERE/PLACE", "space"), p("LIVE", "life"), p("DO", "action")],

        "road" | "path" | "street" | "avenue" | "lane" | "trail"
        | "highway" | "route" | "alley" | "passage" | "corridor"
        | "hallway" | "tunnel" | "canal" | "channel"
        => vec![p("WHERE/PLACE", "space"), p("MOVE", "action")],

        "door" | "window" | "gate" | "entrance" | "exit" | "arch"
        | "threshold" | "portal"
        => vec![p("WHERE/PLACE", "space"), p("MOVE", "action"), p("CAN", "logical")],

        "wall" | "fence" | "barrier" | "dam" | "dike" | "levee"
        => vec![p("WHERE/PLACE", "space"), p("NOT", "logical"), p("MOVE", "action")],

        "floor" | "ceiling" | "roof" | "foundation" | "pillar"
        | "column" | "beam" | "rafter" | "stair" | "step" | "ladder"
        => vec![p("WHERE/PLACE", "space"), p("ABOVE", "space"), p("SOMETHING", "substantive")],

        "field" | "valley" | "plain" | "meadow" | "prairie"
        | "steppe" | "tundra" | "savanna" | "plateau"
        => vec![p("WHERE/PLACE", "space"), p("BIG", "descriptor"), p("LIVE", "life")],

        "desert" | "wasteland" => vec![p("WHERE/PLACE", "space"), p("BIG", "descriptor"), p("NOT", "logical"), p("LIVE", "life")],

        "island" | "peninsula" | "cape" | "coast" | "shore" | "beach"
        | "cliff" | "cove" | "bay" | "reef"
        => vec![p("WHERE/PLACE", "space"), p("SOMETHING", "substantive"), p("MOVE", "action")],

        "hill" | "ridge" | "slope" | "mound" | "dune" | "knoll"
        => vec![p("WHERE/PLACE", "space"), p("ABOVE", "space")],

        "cave" | "cavern" | "grotto" | "burrow" | "hole" | "pit"
        | "well" | "ditch" | "trench" | "gorge" | "canyon" | "ravine"
        | "chasm" | "abyss" | "crater"
        => vec![p("WHERE/PLACE", "space"), p("BELOW", "space"), p("INSIDE", "space")],

        "lake" | "pond" | "pool" | "lagoon" | "reservoir" | "swamp"
        | "marsh" | "bog" | "wetland" | "oasis" | "spring" | "stream"
        | "creek" | "brook" | "waterfall" | "rapids" | "delta"
        => vec![p("WHERE/PLACE", "space"), p("SOMETHING", "substantive"), p("MOVE", "action")],

        "land" | "continent" | "region" | "territory" | "province"
        | "district" | "zone" | "realm" | "domain" | "kingdom"
        | "empire" | "nation" | "homeland" | "frontier" | "wilderness"
        => vec![p("WHERE/PLACE", "space"), p("BIG", "descriptor"), p("PEOPLE", "substantive")],

        // ═══ Tools & Objects ═══
        "knife" | "blade" | "dagger" | "scissors" | "razor"
        | "scalpel" | "chisel" | "awl" | "file" | "drill"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("SOMETHING", "substantive")],

        "sword" | "axe" | "spear" | "lance" | "mace" | "flail"
        | "halberd" | "pike" | "javelin" | "trident"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("BAD", "evaluator"), p("BIG", "descriptor")],

        "bow" | "arrow" | "crossbow" | "sling" | "catapult"
        | "gun" | "rifle" | "pistol" | "cannon" | "bomb"
        | "missile" | "bullet" | "torpedo" | "grenade"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("BAD", "evaluator"), p("FAR", "space")],

        "shield" | "armor" | "helmet" | "gauntlet"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("NOT", "logical"), p("BAD", "evaluator")],

        "hammer" | "saw" | "wrench" | "pliers" | "screwdriver"
        | "shovel" | "rake" | "hoe" | "pickaxe" | "crowbar"
        | "lever" | "wedge" | "plow" | "scythe" | "sickle"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("CAN", "logical")],

        "rope" | "chain" | "wire" | "cable" | "cord" | "knot"
        | "net" | "trap" | "snare" | "lasso" | "leash" | "harness"
        => vec![p("SOMETHING", "substantive"), p("HAVE", "existence"), p("NOT", "logical"), p("MOVE", "action")],

        "key" | "lock" | "bolt" | "latch" | "hinge" | "hook"
        | "clasp" | "buckle" | "zipper" | "button" | "pin" | "nail"
        | "screw" | "rivet"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("CAN", "logical"), p("INSIDE", "space")],

        "wheel" | "gear" | "pulley" | "crank" | "axle" | "spring"
        | "piston" | "valve" | "turbine" | "propeller"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("LIKE/WAY", "similarity")],

        "engine" | "motor" | "generator" | "machine" | "device"
        | "mechanism" | "apparatus" | "instrument" | "gadget"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("MOVE", "action"), p("CAN", "logical")],

        "clock" | "watch" | "sundial" | "hourglass" | "timer"
        | "metronome" | "calendar"
        => vec![p("SOMETHING", "substantive"), p("WHEN/TIME", "time"), p("SEE", "mental")],

        "compass" | "map" | "chart" | "atlas" | "globe"
        => vec![p("SOMETHING", "substantive"), p("WHERE/PLACE", "space"), p("SEE", "mental")],

        "telescope" | "binoculars" | "microscope" | "magnifier"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("FAR", "space")],

        "mirror" | "lens" | "prism"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("LIKE/WAY", "similarity")],

        "candle" | "lamp" | "torch" | "lantern" | "flashlight"
        | "beacon" | "lighthouse"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("GOOD", "evaluator")],

        "bell" | "horn" | "whistle" | "siren" | "gong" | "chime"
        => vec![p("SOMETHING", "substantive"), p("HEAR", "mental"), p("SAY", "speech")],

        "pen" | "pencil" | "quill" | "crayon" | "marker" | "stylus"
        => vec![p("SOMETHING", "substantive"), p("WORDS", "speech"), p("DO", "action")],

        "book" | "page" | "chapter" | "volume" | "manuscript"
        | "document" | "text" | "script" | "note" | "journal"
        | "diary" | "almanac" | "encyclopedia" | "dictionary"
        | "catalog" | "archive" | "record" | "ledger" | "log"
        => vec![p("SOMETHING", "substantive"), p("WORDS", "speech"), p("KNOW", "mental")],

        "letter" | "message" | "mail" | "telegram" | "postcard"
        | "envelope" | "stamp" | "parcel" | "package"
        => vec![p("SOMETHING", "substantive"), p("WORDS", "speech"), p("FAR", "space")],

        "picture" | "painting" | "portrait" | "photograph" | "photo"
        | "image" | "illustration" | "sketch" | "drawing" | "mural"
        | "fresco" | "mosaic" | "sculpture" | "statue" | "carving"
        | "figure" | "icon" | "idol" | "bust"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("DO", "action")],

        "flag" | "banner" | "emblem" | "crest" | "badge" | "medal"
        | "trophy" | "prize" | "award" | "certificate" | "diploma"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "cup" | "bowl" | "plate" | "dish" | "pot" | "pan"
        | "kettle" | "jug" | "pitcher" | "vase" | "jar" | "bottle"
        | "flask" | "goblet" | "mug" | "glass" | "bucket"
        | "barrel" | "tank" | "basin" | "tub" | "cauldron"
        => vec![p("SOMETHING", "substantive"), p("INSIDE", "space"), p("HAVE", "existence")],

        "table" | "desk" | "bench" | "chair" | "stool" | "throne"
        | "couch" | "sofa" | "bed" | "cradle" | "crib" | "shelf"
        | "cabinet" | "drawer" | "closet" | "wardrobe" | "chest"
        => vec![p("SOMETHING", "substantive"), p("WHERE/PLACE", "space"), p("BODY", "substantive")],

        "bag" | "sack" | "pouch" | "purse" | "wallet" | "backpack"
        | "suitcase" | "trunk" | "crate" | "coffin" | "casket"
        => vec![p("SOMETHING", "substantive"), p("INSIDE", "space"), p("HAVE", "existence"), p("MOVE", "action")],

        // ═══ Clothing ═══
        "dress" | "shirt" | "blouse" | "jacket" | "coat" | "cloak"
        | "robe" | "gown" | "tunic" | "vest" | "sweater" | "hoodie"
        | "uniform" | "suit" | "tuxedo" | "skirt" | "pants"
        | "trousers" | "jeans" | "shorts" | "underwear" | "sock"
        | "stocking" | "scarf" | "shawl" | "apron" | "cape"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("ABOVE", "space")],

        "hat" | "cap" | "crown" | "tiara" | "turban" | "bonnet"
        | "hood" | "veil" | "mask" | "headband" | "wreath"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("ABOVE", "space"), p("SEE", "mental")],

        "shoe" | "boot" | "sandal" | "slipper" | "clog"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("BELOW", "space"), p("MOVE", "action")],

        "glove" | "mitten" | "gauntlet"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("DO", "action")],

        "ring" | "bracelet" | "necklace" | "pendant" | "earring"
        | "brooch" | "amulet" | "talisman" | "charm"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("SEE", "mental"), p("GOOD", "evaluator")],

        "belt" | "strap" | "band" | "collar" | "tie"
        => vec![p("SOMETHING", "substantive"), p("BODY", "substantive"), p("HAVE", "existence")],

        // ═══ Vehicles ═══
        "boat" | "canoe" | "kayak" | "raft" | "barge" | "ferry"
        | "yacht" | "sailboat" | "vessel" | "craft" | "ark"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("SOMETHING", "substantive")],

        "wagon" | "cart" | "carriage" | "chariot" | "sled" | "sleigh"
        | "bicycle" | "motorcycle" | "bus" | "truck" | "van"
        | "ambulance" | "taxi" | "limousine"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("CAN", "logical")],

        "train" | "locomotive" | "subway" | "tram" | "trolley"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("CAN", "logical"), p("BIG", "descriptor")],

        "plane" | "airplane" | "helicopter" | "balloon" | "airship"
        | "glider" | "jet" | "rocket" | "spacecraft" | "satellite"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("ABOVE", "space"), p("CAN", "logical")],

        // ═══ Weather & Elements ═══
        "cloud" | "fog" | "mist" | "haze" | "smog"
        => vec![p("SOMETHING", "substantive"), p("ABOVE", "space"), p("SEE", "mental"), p("NOT", "logical")],

        "storm" | "tempest" | "hurricane" | "tornado" | "typhoon"
        | "cyclone" | "blizzard" | "thunderstorm"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("BIG", "descriptor"), p("BAD", "evaluator")],

        "thunder" => vec![p("HEAR", "mental"), p("BIG", "descriptor"), p("ABOVE", "space")],
        "lightning" => vec![p("SEE", "mental"), p("MOVE", "action"), p("ABOVE", "space")],
        "rainbow" => vec![p("SEE", "mental"), p("GOOD", "evaluator"), p("ABOVE", "space")],
        "flood" => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("MUCH/MANY", "quantifier"), p("BAD", "evaluator")],
        "drought" => vec![p("NOT", "logical"), p("SOMETHING", "substantive"), p("A LONG TIME", "time"), p("BAD", "evaluator")],
        "frost" | "ice" | "hail" | "sleet"
        => vec![p("SOMETHING", "substantive"), p("FEEL", "mental"), p("NOT", "logical"), p("MOVE", "action")],
        "dew" => vec![p("SOMETHING", "substantive"), p("SMALL", "descriptor"), p("NOW", "time")],

        // ═══ Colors ═══
        "red" | "scarlet" | "crimson" | "maroon" | "vermilion"
        => vec![p("SEE", "mental"), p("FEEL", "mental"), p("BIG", "descriptor")],

        "blue" | "azure" | "cobalt" | "navy" | "indigo" | "cerulean"
        => vec![p("SEE", "mental"), p("ABOVE", "space"), p("FAR", "space")],

        "green" | "emerald" | "jade" | "olive" | "lime" | "teal"
        => vec![p("SEE", "mental"), p("LIVE", "life"), p("GOOD", "evaluator")],

        "yellow" | "golden" | "amber" | "saffron" | "ochre"
        => vec![p("SEE", "mental"), p("ABOVE", "space"), p("GOOD", "evaluator")],

        "black" | "ebony" | "onyx" | "obsidian" | "jet"
        => vec![p("SEE", "mental"), p("NOT", "logical")],

        "white" | "ivory" | "snow" | "pearl" | "cream"
        => vec![p("SEE", "mental"), p("ALL", "quantifier")],

        "purple" | "violet" | "magenta" | "plum" | "mauve" | "lavender"
        => vec![p("SEE", "mental"), p("ABOVE", "space"), p("FEEL", "mental")],

        "pink" | "rose" | "blush" | "salmon" | "coral"
        => vec![p("SEE", "mental"), p("FEEL", "mental"), p("GOOD", "evaluator")],

        "brown" | "tan" | "beige" | "khaki" | "sienna" | "umber"
        => vec![p("SEE", "mental"), p("BELOW", "space"), p("SOMETHING", "substantive")],

        "gray" | "grey" | "silver" | "charcoal" | "slate" | "ash"
        => vec![p("SEE", "mental"), p("OTHER", "determiner")],

        "orange" | "tangerine" | "rust" | "copper" | "bronze"
        => vec![p("SEE", "mental"), p("FEEL", "mental"), p("SOMETHING", "substantive")],

        // ═══ People & Professions ═══
        "doctor" | "physician" | "surgeon" | "nurse" | "healer"
        | "medic" | "dentist" | "therapist" | "paramedic"
        => vec![p("SOMEONE", "substantive"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "soldier" | "warrior" | "knight" | "guard" | "sentinel"
        | "ranger" | "mercenary" | "gladiator" | "samurai" | "viking"
        => vec![p("SOMEONE", "substantive"), p("DO", "action"), p("BAD", "evaluator"), p("CAN", "logical")],

        "farmer" | "shepherd" | "gardener" | "fisherman"
        | "lumberjack" | "woodcutter"
        => vec![p("SOMEONE", "substantive"), p("DO", "action"), p("LIVE", "life")],

        "sailor" | "navigator" | "captain" | "pilot"
        => vec![p("SOMEONE", "substantive"), p("MOVE", "action"), p("FAR", "space")],

        "merchant" | "trader" | "vendor" | "shopkeeper" | "banker"
        | "broker" | "dealer"
        => vec![p("SOMEONE", "substantive"), p("HAVE", "existence"), p("DO", "action")],

        "priest" | "monk" | "nun" | "bishop" | "pope" | "imam"
        | "rabbi" | "shaman" | "prophet" | "saint" | "apostle"
        | "disciple" | "pilgrim" | "missionary"
        => vec![p("SOMEONE", "substantive"), p("ABOVE", "space"), p("FEEL", "mental"), p("GOOD", "evaluator")],

        "judge" | "lawyer" | "attorney" | "advocate" | "magistrate"
        => vec![p("SOMEONE", "substantive"), p("THINK", "mental"), p("GOOD", "evaluator"), p("BAD", "evaluator")],

        "artist" | "painter" | "sculptor" | "photographer"
        => vec![p("SOMEONE", "substantive"), p("SEE", "mental"), p("DO", "action")],

        "writer" | "author" | "poet" | "novelist" | "playwright"
        | "journalist" | "reporter" | "editor" | "scribe"
        => vec![p("SOMEONE", "substantive"), p("WORDS", "speech"), p("DO", "action")],

        "singer" | "musician" | "composer" | "conductor" | "performer"
        => vec![p("SOMEONE", "substantive"), p("HEAR", "mental"), p("DO", "action"), p("GOOD", "evaluator")],

        "dancer" => vec![p("SOMEONE", "substantive"), p("MOVE", "action"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "hunter" | "tracker" | "stalker" | "predator"
        => vec![p("SOMEONE", "substantive"), p("WANT", "mental"), p("MOVE", "action")],

        "cook" | "chef" | "baker" | "butcher" | "brewer"
        => vec![p("SOMEONE", "substantive"), p("DO", "action"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "servant" | "slave" | "maid" | "butler" | "attendant"
        => vec![p("SOMEONE", "substantive"), p("DO", "action"), p("OTHER", "determiner")],

        "spy" | "scout" | "assassin"
        => vec![p("SOMEONE", "substantive"), p("SEE", "mental"), p("NOT", "logical"), p("KNOW", "mental")],

        "thief" | "robber" | "pirate" | "bandit" | "outlaw"
        | "criminal" | "villain" | "rogue" | "scoundrel"
        => vec![p("SOMEONE", "substantive"), p("HAVE", "existence"), p("BAD", "evaluator")],

        "hero" | "champion" | "savior"
        => vec![p("SOMEONE", "substantive"), p("DO", "action"), p("GOOD", "evaluator"), p("BIG", "descriptor")],

        "fool" | "idiot" | "clown" | "jester"
        => vec![p("SOMEONE", "substantive"), p("NOT", "logical"), p("THINK", "mental")],

        "stranger" | "foreigner" | "alien" | "outsider" | "exile"
        | "refugee" | "immigrant" | "nomad" | "wanderer" | "vagabond"
        => vec![p("SOMEONE", "substantive"), p("OTHER", "determiner"), p("FAR", "space")],

        "prisoner" | "captive" | "hostage"
        => vec![p("SOMEONE", "substantive"), p("NOT", "logical"), p("CAN", "logical"), p("MOVE", "action")],

        "emperor" | "chief" | "lord" | "master" | "commander"
        | "ruler" | "monarch" | "sultan" | "pharaoh" | "tsar"
        | "president" | "governor" | "mayor" | "general" | "admiral"
        => vec![p("SOMEONE", "substantive"), p("BIG", "descriptor"), p("ABOVE", "space"), p("PEOPLE", "substantive")],

        "prince" | "princess" | "duke" | "duchess" | "baron"
        | "count" | "earl" | "knight" | "squire" | "noble"
        => vec![p("SOMEONE", "substantive"), p("ABOVE", "space"), p("PEOPLE", "substantive")],

        "baby" | "infant" | "toddler" | "boy" | "girl"
        | "teenager" | "youth" | "maiden" | "lad" | "lass"
        => vec![p("SOMEONE", "substantive"), p("SMALL", "descriptor"), p("LIVE", "life")],

        "elder" | "ancestor" | "patriarch" | "matriarch"
        | "grandparent" | "grandmother" | "grandfather"
        => vec![p("SOMEONE", "substantive"), p("LIVE", "life"), p("BEFORE", "time"), p("A LONG TIME", "time")],

        "brother" | "sister" | "sibling" | "twin" | "cousin"
        | "nephew" | "niece" | "uncle" | "aunt" | "relative"
        => vec![p("SOMEONE", "substantive"), p("THE SAME", "determiner"), p("LIVE", "life")],

        "husband" | "wife" | "spouse" | "partner" | "lover"
        | "bride" | "groom" | "fiancé" | "fiancée"
        | "companion" | "mate"
        => vec![p("SOMEONE", "substantive"), p("FEEL", "mental"), p("GOOD", "evaluator"), p("LIVE", "life")],

        "neighbor" | "ally" | "comrade" | "colleague" | "companion"
        => vec![p("SOMEONE", "substantive"), p("NEAR", "space"), p("GOOD", "evaluator")],

        "enemy" | "foe" | "rival" | "opponent" | "adversary"
        | "nemesis"
        => vec![p("SOMEONE", "substantive"), p("BAD", "evaluator"), p("DO", "action")],

        "crowd" | "mob" | "army" | "herd" | "flock" | "pack"
        | "swarm" | "tribe" | "clan" | "gang" | "crew" | "troop"
        | "squad" | "team" | "group" | "band" | "congregation"
        => vec![p("PEOPLE", "substantive"), p("MUCH/MANY", "quantifier")],

        "ghost" | "phantom" | "specter" | "wraith" | "shade"
        | "apparition" | "demon" | "devil" | "angel" | "fairy"
        | "elf" | "dwarf" | "troll" | "goblin" | "witch" | "wizard"
        | "sorcerer" | "mage"
        => vec![p("SOMEONE", "substantive"), p("NOT", "logical"), p("BODY", "substantive")],

        // ═══ Music & Art ═══
        "guitar" | "piano" | "violin" | "cello" | "harp" | "lute"
        | "banjo" | "ukulele" | "mandolin" | "organ" | "accordion"
        | "harmonica" | "flute" | "clarinet" | "trumpet" | "trombone"
        | "saxophone" | "oboe" | "bassoon" | "bagpipe" | "drum"
        | "cymbal" | "tambourine" | "xylophone"
        => vec![p("SOMETHING", "substantive"), p("HEAR", "mental"), p("DO", "action"), p("GOOD", "evaluator")],

        "poem" | "verse" | "rhyme" | "ballad" | "hymn" | "anthem"
        | "lullaby" | "chant" | "melody" | "harmony" | "rhythm"
        | "chord" | "note" | "tune" | "lyric" | "chorus" | "refrain"
        | "symphony" | "opera" | "sonata" | "concerto" | "overture"
        => vec![p("HEAR", "mental"), p("FEEL", "mental"), p("WORDS", "speech"), p("GOOD", "evaluator")],

        "theater" | "stage" | "scene" | "act" | "play" | "drama"
        | "comedy" | "tragedy" | "farce" | "show" | "performance"
        | "concert" | "recital" | "festival" | "carnival" | "circus"
        => vec![p("SEE", "mental"), p("HEAR", "mental"), p("PEOPLE", "substantive"), p("FEEL", "mental")],

        "dance" => vec![p("MOVE", "action"), p("BODY", "substantive"), p("HEAR", "mental"), p("GOOD", "evaluator")],

        // ═══ Science & Math ═══
        "atom" | "molecule" | "electron" | "proton" | "neutron"
        | "quark" | "photon" | "ion" | "isotope"
        => vec![p("SOMETHING", "substantive"), p("SMALL", "descriptor"), p("SOMETHING", "substantive")],

        "cell" | "gene" | "chromosome" | "dna" | "protein"
        | "enzyme" | "bacteria" | "virus" | "microbe" | "organism"
        => vec![p("SOMETHING", "substantive"), p("LIVE", "life"), p("SMALL", "descriptor")],

        "gravity" | "magnetism" | "electricity" | "radiation"
        | "friction" | "pressure" | "tension" | "momentum" | "inertia"
        => vec![p("SOMETHING", "substantive"), p("MOVE", "action"), p("CAN", "logical")],

        "equation" | "formula" | "theorem" | "proof" | "hypothesis"
        | "theory" | "law" | "principle" | "axiom" | "paradox"
        => vec![p("THINK", "mental"), p("KNOW", "mental"), p("TRUE", "speech")],

        "experiment" | "test" | "trial" | "observation" | "analysis"
        | "research" | "study" | "survey" | "census" | "investigation"
        => vec![p("DO", "action"), p("SEE", "mental"), p("KNOW", "mental")],

        "number" | "zero" | "three" | "four" | "five" | "six"
        | "seven" | "eight" | "nine" | "ten" | "hundred" | "thousand"
        | "million" | "billion" | "dozen" | "score" | "pair"
        => vec![p("SOMETHING", "substantive"), p("MUCH/MANY", "quantifier")],

        "half" | "quarter" | "third" | "fraction" | "percent"
        | "ratio" | "proportion"
        => vec![p("SOMETHING", "substantive"), p("SOMETHING", "substantive"), p("LIKE/WAY", "similarity")],

        // ═══ Society & Government ═══
        "law" | "rule" | "regulation" | "decree" | "edict"
        | "mandate" | "statute" | "ordinance" | "commandment"
        => vec![p("SAY", "speech"), p("DO", "action"), p("PEOPLE", "substantive")],

        "crime" | "sin" | "offense" | "violation" | "transgression"
        => vec![p("DO", "action"), p("BAD", "evaluator"), p("NOT", "logical"), p("GOOD", "evaluator")],

        "justice" | "fairness" | "equity"
        => vec![p("GOOD", "evaluator"), p("THE SAME", "determiner"), p("PEOPLE", "substantive")],

        "mercy" | "forgiveness" | "compassion" | "pity" | "sympathy"
        | "empathy" | "kindness" | "charity" | "generosity"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("SOMEONE", "substantive"), p("BAD", "evaluator")],

        "honor" | "glory" | "fame" | "reputation" | "prestige"
        | "dignity" | "respect" | "admiration" | "esteem"
        => vec![p("GOOD", "evaluator"), p("SOMEONE", "substantive"), p("PEOPLE", "substantive"), p("THINK", "mental")],

        "shame" | "disgrace" | "humiliation" | "dishonor"
        => vec![p("BAD", "evaluator"), p("SOMEONE", "substantive"), p("PEOPLE", "substantive"), p("FEEL", "mental")],

        "duty" | "obligation" | "responsibility"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("CAN", "logical")],

        "right" | "privilege" | "entitlement" | "liberty"
        => vec![p("CAN", "logical"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "tax" | "debt" | "fine" | "penalty" | "ransom" | "tribute"
        => vec![p("HAVE", "existence"), p("NOT", "logical"), p("DO", "action")],

        "oath" | "vow" | "pledge" | "promise" | "covenant"
        | "contract" | "treaty" | "agreement" | "pact" | "alliance"
        => vec![p("SAY", "speech"), p("TRUE", "speech"), p("DO", "action")],

        "sacrifice" | "offering"
        => vec![p("HAVE", "existence"), p("NOT", "logical"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "gift" | "present" | "donation" | "reward" | "bonus"
        => vec![p("HAVE", "existence"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "punishment" | "penalty" | "sentence" | "exile" | "banishment"
        => vec![p("DO", "action"), p("BAD", "evaluator"), p("SOMEONE", "substantive")],

        "victory" | "triumph" | "conquest" | "success"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("OTHER", "determiner")],

        "defeat" | "failure" | "loss" | "ruin" | "collapse"
        | "disaster" | "catastrophe" | "calamity" | "tragedy"
        => vec![p("HAPPEN", "action"), p("BAD", "evaluator"), p("BIG", "descriptor")],

        "revolution" | "rebellion" | "revolt" | "uprising" | "mutiny"
        | "coup" | "insurgency" | "resistance"
        => vec![p("DO", "action"), p("NOT", "logical"), p("ABOVE", "space"), p("PEOPLE", "substantive")],

        "money" | "coin" | "currency" | "cash" | "wealth" | "fortune"
        | "riches" | "inheritance" | "dowry"
        => vec![p("SOMETHING", "substantive"), p("HAVE", "existence"), p("MUCH/MANY", "quantifier"), p("WANT", "mental")],

        "vote" | "election" | "ballot" | "referendum"
        => vec![p("SAY", "speech"), p("WANT", "mental"), p("PEOPLE", "substantive")],

        // ═══ Abstract Concepts ═══
        "fate" | "destiny" | "fortune" | "karma" | "providence"
        => vec![p("HAPPEN", "action"), p("BEFORE", "time"), p("CAN", "logical"), p("NOT", "logical")],

        "chaos" | "disorder" | "anarchy" | "entropy"
        => vec![p("NOT", "logical"), p("LIKE/WAY", "similarity"), p("MOVE", "action")],

        "order" | "structure" | "system" | "arrangement" | "hierarchy"
        => vec![p("LIKE/WAY", "similarity"), p("WHERE/PLACE", "space"), p("GOOD", "evaluator")],

        "balance" | "equilibrium" | "symmetry"
        => vec![p("THE SAME", "determiner"), p("LIKE/WAY", "similarity"), p("GOOD", "evaluator")],

        "conflict" | "struggle" | "strife" | "tension" | "friction"
        => vec![p("DO", "action"), p("NOT", "logical"), p("THE SAME", "determiner"), p("BAD", "evaluator")],

        "mystery" | "enigma" | "riddle" | "puzzle"
        => vec![p("NOT", "logical"), p("KNOW", "mental"), p("WANT", "mental"), p("KNOW", "mental")],

        "secret" => vec![p("KNOW", "mental"), p("NOT", "logical"), p("OTHER", "determiner"), p("WANT", "mental")],

        "miracle" | "wonder" | "marvel"
        => vec![p("HAPPEN", "action"), p("GOOD", "evaluator"), p("NOT", "logical"), p("CAN", "logical")],

        "curse" | "hex" | "spell" | "enchantment" | "incantation"
        => vec![p("SAY", "speech"), p("DO", "action"), p("BAD", "evaluator")],

        "blessing" | "benediction"
        => vec![p("SAY", "speech"), p("DO", "action"), p("GOOD", "evaluator")],

        "luck" | "chance" | "coincidence" | "accident"
        => vec![p("HAPPEN", "action"), p("NOT", "logical"), p("BECAUSE", "logical")],

        "virtue" | "goodness" | "righteousness" | "integrity"
        | "honesty" | "sincerity" | "loyalty" | "faithfulness"
        | "devotion" | "dedication"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "sin" | "wickedness" | "corruption" | "greed" | "envy"
        | "vanity" | "arrogance" | "cruelty" | "malice"
        | "treachery" | "betrayal" | "deceit" | "deception"
        => vec![p("DO", "action"), p("BAD", "evaluator"), p("WANT", "mental")],

        "faith" | "belief" | "trust" | "confidence" | "conviction"
        => vec![p("THINK", "mental"), p("TRUE", "speech"), p("GOOD", "evaluator")],

        "doubt" | "uncertainty" | "skepticism" | "suspicion"
        => vec![p("THINK", "mental"), p("NOT", "logical"), p("TRUE", "speech")],

        "danger" | "threat" | "peril" | "hazard" | "risk"
        => vec![p("HAPPEN", "action"), p("BAD", "evaluator"), p("CAN", "logical")],

        "safety" | "security" | "protection" | "shelter" | "refuge"
        => vec![p("NOT", "logical"), p("BAD", "evaluator"), p("HAPPEN", "action")],

        "idea" | "concept" | "notion" | "thought" | "theory"
        => vec![p("THINK", "mental"), p("SOMETHING", "substantive")],

        "plan" | "strategy" | "scheme" | "design" | "blueprint"
        | "method" | "technique" | "procedure" | "process" | "recipe"
        => vec![p("THINK", "mental"), p("DO", "action"), p("BEFORE", "time")],

        "reason" | "logic" | "argument" | "evidence" | "proof"
        => vec![p("THINK", "mental"), p("BECAUSE", "logical"), p("TRUE", "speech")],

        "game" | "sport" | "contest" | "match" | "race" | "tournament"
        | "competition" | "challenge" | "duel"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("OTHER", "determiner")],

        "toy" | "doll" | "puppet" | "kite" | "ball"
        => vec![p("SOMETHING", "substantive"), p("DO", "action"), p("GOOD", "evaluator"), p("SMALL", "descriptor")],

        "price" | "cost" | "value" | "worth"
        => vec![p("SOMETHING", "substantive"), p("MUCH/MANY", "quantifier"), p("HAVE", "existence")],

        "age" | "era" | "epoch" | "period" | "generation"
        | "decade" | "century" | "millennium"
        => vec![p("WHEN/TIME", "time"), p("A LONG TIME", "time"), p("BIG", "descriptor")],

        "tradition" | "custom" | "ritual" | "ceremony" | "rite"
        | "celebration" | "festival" | "holiday" | "feast"
        => vec![p("DO", "action"), p("BEFORE", "time"), p("PEOPLE", "substantive"), p("THE SAME", "determiner")],

        "legend" | "myth" | "fable" | "parable" | "saga" | "epic"
        | "tale" | "folklore" | "lore"
        => vec![p("SAY", "speech"), p("BEFORE", "time"), p("PEOPLE", "substantive")],

        "prayer" | "meditation" | "worship" | "devotion"
        => vec![p("SAY", "speech"), p("FEEL", "mental"), p("ABOVE", "space")],

        "religion" | "theology" | "philosophy" | "ideology"
        | "doctrine" | "dogma" | "creed"
        => vec![p("THINK", "mental"), p("ABOVE", "space"), p("PEOPLE", "substantive")],

        // ═══ Emotions & States (beyond what language.rs covers) ═══
        "surprise" | "shock" | "amazement" | "astonishment" | "awe"
        => vec![p("FEEL", "mental"), p("NOT", "logical"), p("KNOW", "mental"), p("HAPPEN", "action")],

        "pride" => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("I", "substantive")],
        "shame" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("I", "substantive"), p("PEOPLE", "substantive")],
        "guilt" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("DO", "action"), p("I", "substantive")],
        "jealousy" | "envy" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("WANT", "mental"), p("HAVE", "existence")],
        "loneliness" | "solitude" => vec![p("FEEL", "mental"), p("NOT", "logical"), p("SOMEONE", "substantive"), p("NEAR", "space")],
        "courage" | "bravery" | "valor" => vec![p("FEEL", "mental"), p("NOT", "logical"), p("BAD", "evaluator"), p("DO", "action")],
        "cowardice" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("NOT", "logical"), p("DO", "action")],
        "patience" => vec![p("NOT", "logical"), p("DO", "action"), p("A LONG TIME", "time"), p("GOOD", "evaluator")],
        "hunger" => vec![p("FEEL", "mental"), p("BODY", "substantive"), p("WANT", "mental")],
        "thirst" => vec![p("FEEL", "mental"), p("BODY", "substantive"), p("WANT", "mental"), p("SOMETHING", "substantive")],
        "desire" | "craving" | "longing" | "yearning"
        => vec![p("WANT", "mental"), p("MUCH/MANY", "quantifier"), p("FEEL", "mental")],
        "pleasure" | "delight" | "bliss" | "ecstasy" | "euphoria"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("MUCH/MANY", "quantifier"), p("BODY", "substantive")],
        "agony" | "torment" | "anguish" | "misery" | "despair"
        => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("MUCH/MANY", "quantifier")],
        "confusion" | "bewilderment" | "perplexity"
        => vec![p("THINK", "mental"), p("NOT", "logical"), p("KNOW", "mental")],
        "nostalgia" => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("BEFORE", "time"), p("WANT", "mental")],
        "regret" | "remorse" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("BEFORE", "time"), p("DO", "action")],
        "gratitude" | "thankfulness" => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("SOMEONE", "substantive"), p("DO", "action")],
        "contentment" | "serenity" | "tranquility"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("NOT", "logical"), p("WANT", "mental")],

        // ═══ Actions (verbs not in language.rs primes) ═══
        "attack" | "assault" | "strike" | "hit" | "punch" | "kick"
        | "slap" | "stab" | "slash" | "smash" | "crush" | "pound"
        => vec![p("DO", "action"), p("BAD", "evaluator"), p("BODY", "substantive")],

        "defend" | "protect" | "guard" | "shield"
        => vec![p("DO", "action"), p("NOT", "logical"), p("BAD", "evaluator")],

        "escape" | "flee" | "retreat"
        => vec![p("MOVE", "action"), p("FAR", "space"), p("BAD", "evaluator")],

        "hide" | "conceal" | "disguise" | "camouflage"
        => vec![p("NOT", "logical"), p("SEE", "mental"), p("DO", "action")],

        "chase" | "pursue" | "hunt" | "stalk" | "track"
        => vec![p("MOVE", "action"), p("WANT", "mental"), p("SOMEONE", "substantive")],

        "climb" | "ascend" | "soar" | "leap" | "jump" | "bounce"
        => vec![p("MOVE", "action"), p("ABOVE", "space"), p("BODY", "substantive")],

        "dig" | "excavate" | "burrow" | "tunnel"
        => vec![p("DO", "action"), p("BELOW", "space"), p("MOVE", "action")],

        "pour" | "spill" | "drip" | "leak" | "spray" | "splash"
        | "flood" | "drown" | "soak" | "drench"
        => vec![p("MOVE", "action"), p("SOMETHING", "substantive"), p("BELOW", "space")],

        "burn" | "ignite" | "kindle" | "scorch" | "singe" | "char"
        | "blaze" | "smolder" | "glow" | "flare" | "flash"
        => vec![p("SOMETHING", "substantive"), p("SEE", "mental"), p("FEEL", "mental"), p("BAD", "evaluator")],

        "freeze" | "chill" | "cool"
        => vec![p("NOT", "logical"), p("MOVE", "action"), p("FEEL", "mental")],

        "melt" | "thaw" | "dissolve"
        => vec![p("MOVE", "action"), p("SOMETHING", "substantive"), p("OTHER", "determiner")],

        "heal" | "cure" | "mend" | "repair" | "restore" | "fix"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("BAD", "evaluator"), p("NOT", "logical")],

        "breathe" | "inhale" | "exhale" | "gasp" | "sigh" | "pant"
        => vec![p("LIVE", "life"), p("BODY", "substantive"), p("MOVE", "action")],

        "whisper" | "murmur" | "mumble" | "mutter"
        => vec![p("SAY", "speech"), p("SMALL", "descriptor"), p("HEAR", "mental")],

        "shout" | "scream" | "yell" | "roar" | "howl" | "shriek"
        | "wail" | "cry"
        => vec![p("SAY", "speech"), p("BIG", "descriptor"), p("HEAR", "mental")],

        "laugh" | "giggle" | "chuckle" | "snicker" | "cackle"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("HEAR", "mental"), p("BODY", "substantive")],

        "smile" | "grin" | "beam" | "smirk"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("SEE", "mental"), p("BODY", "substantive")],

        "weep" | "sob" | "mourn" | "grieve" | "lament"
        => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("BODY", "substantive")],

        "kiss" | "embrace" | "hug" | "caress" | "cuddle"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("BODY", "substantive"), p("NEAR", "space")],

        "pray" | "beg" | "plead" | "implore"
        => vec![p("SAY", "speech"), p("WANT", "mental"), p("ABOVE", "space")],

        "curse" | "swear" | "damn" | "condemn"
        => vec![p("SAY", "speech"), p("BAD", "evaluator"), p("FEEL", "mental")],

        "bless" | "consecrate" | "sanctify" | "anoint"
        => vec![p("SAY", "speech"), p("GOOD", "evaluator"), p("ABOVE", "space")],

        "forgive" | "pardon" | "absolve"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("BAD", "evaluator"), p("NOT", "logical")],

        "punish" | "discipline" | "penalize"
        => vec![p("DO", "action"), p("BAD", "evaluator"), p("BECAUSE", "logical")],

        "judge" | "evaluate" | "assess" | "examine"
        => vec![p("THINK", "mental"), p("GOOD", "evaluator"), p("BAD", "evaluator")],

        "govern" | "reign" | "command" | "direct" | "manage"
        | "supervise" | "oversee" | "administer" | "regulate"
        => vec![p("SAY", "speech"), p("DO", "action"), p("PEOPLE", "substantive"), p("ABOVE", "space")],

        "obey" | "comply" | "submit" | "yield" | "surrender"
        | "capitulate"
        => vec![p("DO", "action"), p("SOMEONE", "substantive"), p("SAY", "speech")],

        "rebel" | "resist" | "defy" | "disobey" | "refuse"
        | "reject" | "deny"
        => vec![p("NOT", "logical"), p("DO", "action"), p("SOMEONE", "substantive"), p("SAY", "speech")],

        "conquer" | "vanquish" | "subjugate" | "dominate" | "overpower"
        => vec![p("DO", "action"), p("CAN", "logical"), p("OTHER", "determiner"), p("BIG", "descriptor")],

        "rescue" | "save" | "liberate" | "free" | "release"
        => vec![p("DO", "action"), p("GOOD", "evaluator"), p("CAN", "logical"), p("SOMEONE", "substantive")],

        "steal" | "rob" | "plunder" | "loot" | "pillage" | "raid"
        => vec![p("HAVE", "existence"), p("BAD", "evaluator"), p("NOT", "logical"), p("SOMEONE", "substantive")],

        "explore" | "discover" | "uncover" | "reveal" | "expose"
        => vec![p("SEE", "mental"), p("KNOW", "mental"), p("BEFORE", "time"), p("NOT", "logical")],

        "invent" | "devise" | "innovate" | "pioneer"
        => vec![p("THINK", "mental"), p("DO", "action"), p("BEFORE", "time"), p("NOT", "logical")],

        "measure" | "weigh" | "calculate" | "compute" | "estimate"
        => vec![p("KNOW", "mental"), p("MUCH/MANY", "quantifier"), p("SOMETHING", "substantive")],

        "predict" | "forecast" | "foresee" | "prophesy" | "foretell"
        => vec![p("THINK", "mental"), p("AFTER", "time"), p("KNOW", "mental")],

        "solve" | "resolve" | "unravel" | "decipher"
        => vec![p("THINK", "mental"), p("KNOW", "mental"), p("GOOD", "evaluator")],

        "trick" | "deceive" | "cheat" | "swindle" | "dupe"
        => vec![p("SAY", "speech"), p("NOT", "logical"), p("TRUE", "speech"), p("DO", "action")],

        "betray" => vec![p("DO", "action"), p("BAD", "evaluator"), p("FEEL", "mental"), p("GOOD", "evaluator"), p("NOT", "logical")],

        "murder" | "kill" | "slay" | "assassinate" | "execute"
        | "slaughter" | "massacre"
        => vec![p("DO", "action"), p("DIE", "life"), p("SOMEONE", "substantive")],

        "torture" | "torment" => vec![p("DO", "action"), p("FEEL", "mental"), p("BAD", "evaluator"), p("MUCH/MANY", "quantifier")],

        "threaten" | "intimidate" | "menace"
        => vec![p("SAY", "speech"), p("BAD", "evaluator"), p("HAPPEN", "action"), p("CAN", "logical")],

        "warn" | "alert" | "notify" | "signal"
        => vec![p("SAY", "speech"), p("KNOW", "mental"), p("BAD", "evaluator"), p("CAN", "logical")],

        "celebrate" | "rejoice" | "revel"
        => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("DO", "action"), p("PEOPLE", "substantive")],

        "suffer" => vec![p("FEEL", "mental"), p("BAD", "evaluator"), p("A LONG TIME", "time")],

        "sacrifice" => vec![p("HAVE", "existence"), p("NOT", "logical"), p("GOOD", "evaluator"), p("SOMEONE", "substantive")],

        "worship" => vec![p("FEEL", "mental"), p("GOOD", "evaluator"), p("ABOVE", "space"), p("BIG", "descriptor")],

        "meditate" | "contemplate" | "ponder" | "reflect" | "muse"
        => vec![p("THINK", "mental"), p("A LONG TIME", "time"), p("INSIDE", "space")],

        "imagine" | "envision" | "fantasize" | "visualize"
        => vec![p("THINK", "mental"), p("SEE", "mental"), p("NOT", "logical"), p("TRUE", "speech")],

        "choose" | "decide" | "select" | "pick" | "elect"
        => vec![p("THINK", "mental"), p("WANT", "mental"), p("DO", "action")],

        "promise" | "swear" | "vow" | "pledge" | "guarantee"
        => vec![p("SAY", "speech"), p("DO", "action"), p("AFTER", "time"), p("TRUE", "speech")],

        "doubt" | "question" | "wonder" | "suspect"
        => vec![p("THINK", "mental"), p("NOT", "logical"), p("KNOW", "mental")],

        "agree" | "accept" | "approve" | "consent" | "concur"
        => vec![p("THINK", "mental"), p("THE SAME", "determiner"), p("GOOD", "evaluator")],

        "argue" | "debate" | "dispute" | "quarrel" | "bicker"
        => vec![p("SAY", "speech"), p("NOT", "logical"), p("THE SAME", "determiner")],

        "blame" | "accuse" | "charge" | "indict"
        => vec![p("SAY", "speech"), p("DO", "action"), p("BAD", "evaluator"), p("SOMEONE", "substantive")],

        "confess" | "admit" | "acknowledge"
        => vec![p("SAY", "speech"), p("TRUE", "speech"), p("DO", "action"), p("BAD", "evaluator")],

        "sing" => vec![p("SAY", "speech"), p("HEAR", "mental"), p("GOOD", "evaluator"), p("FEEL", "mental")],
        "paint" => vec![p("DO", "action"), p("SEE", "mental"), p("SOMETHING", "substantive")],
        "sculpt" | "carve" | "engrave" | "etch"
        => vec![p("DO", "action"), p("SOMETHING", "substantive"), p("SEE", "mental")],

        "cook" | "bake" | "boil" | "roast" | "fry" | "grill"
        | "stew" | "simmer" | "brew"
        => vec![p("DO", "action"), p("SOMETHING", "substantive"), p("BODY", "substantive"), p("GOOD", "evaluator")],

        "sew" | "knit" | "weave" | "spin" | "braid" | "embroider"
        => vec![p("DO", "action"), p("SOMETHING", "substantive"), p("BODY", "substantive")],

        "plant" | "sow" | "cultivate" | "harvest" | "reap"
        => vec![p("DO", "action"), p("LIVE", "life"), p("SOMETHING", "substantive")],

        "build" | "construct" | "assemble" | "erect" | "raise"
        => vec![p("DO", "action"), p("SOMETHING", "substantive"), p("BIG", "descriptor")],

        _ => return None,
    })
}
