use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyTerm {
    pub word: String,
    pub definition: String,
    pub related_concepts: Vec<String>,
}

pub fn get_physics_vocabulary() -> Vec<VocabularyTerm> {
    vec![
        VocabularyTerm {
            word: "Velocity".to_string(),
            definition: "The speed of something in a given direction.".to_string(),
            related_concepts: vec!["Speed".to_string(), "Direction".to_string(), "Vector".to_string()],
        },
        VocabularyTerm {
            word: "Acceleration".to_string(),
            definition: "The rate of change of velocity per unit of time.".to_string(),
            related_concepts: vec!["Velocity".to_string(), "Time".to_string(), "Force".to_string()],
        },
        VocabularyTerm {
            word: "Inertia".to_string(),
            definition: "A property of matter by which it continues in its existing state of rest or uniform motion in a straight line, unless that state is changed by an external force.".to_string(),
            related_concepts: vec!["Mass".to_string(), "Newton's First Law".to_string()],
        },
        VocabularyTerm {
            word: "Friction".to_string(),
            definition: "The resistance that one surface or object encounters when moving over another.".to_string(),
            related_concepts: vec!["Resistance".to_string(), "Heat".to_string(), "Surface".to_string()],
        },
        VocabularyTerm {
            word: "Momentum".to_string(),
            definition: "The quantity of motion of a moving body, measured as a product of its mass and velocity.".to_string(),
            related_concepts: vec!["Mass".to_string(), "Velocity".to_string(), "Collision".to_string()],
        },
        VocabularyTerm {
            word: "Force".to_string(),
            definition: "Strength or energy as an attribute of physical action or movement.".to_string(),
            related_concepts: vec!["Newton".to_string(), "Interaction".to_string()],
        },
        VocabularyTerm {
            word: "Gravity".to_string(),
            definition: "The force that attracts a body toward the center of the earth, or toward any other physical body having mass.".to_string(),
            related_concepts: vec!["Mass".to_string(), "Weight".to_string(), "Attraction".to_string()],
        },
        VocabularyTerm {
            word: "Mass".to_string(),
            definition: "A coherent, typically large body of matter with no definite shape.".to_string(),
            related_concepts: vec!["Weight".to_string(), "Inertia".to_string(), "Matter".to_string()],
        },
        VocabularyTerm {
            word: "Energy".to_string(),
            definition: "The strength and vitality required for sustained physical or mental activity.".to_string(),
            related_concepts: vec!["Work".to_string(), "Power".to_string(), "Potential".to_string(), "Kinetic".to_string()],
        },
        VocabularyTerm {
            word: "Kinetic Energy".to_string(),
            definition: "Energy which a body possesses by virtue of being in motion.".to_string(),
            related_concepts: vec!["Motion".to_string(), "Work".to_string()],
        },
        VocabularyTerm {
            word: "Potential Energy".to_string(),
            definition: "The energy possessed by a body by virtue of its position relative to others, stresses within itself, electric charge, and other factors.".to_string(),
            related_concepts: vec!["Position".to_string(), "Stored".to_string()],
        },
        VocabularyTerm {
            word: "Work".to_string(),
            definition: "Activity involving mental or physical effort done in order to achieve a purpose or result.".to_string(),
            related_concepts: vec!["Force".to_string(), "Distance".to_string(), "Energy".to_string()],
        },
        VocabularyTerm {
            word: "Power".to_string(),
            definition: "The ability to do something or act in a particular way, especially as a faculty or quality.".to_string(),
            related_concepts: vec!["Work".to_string(), "Time".to_string(), "Watt".to_string()],
        },
        VocabularyTerm {
            word: "Density".to_string(),
            definition: "The degree of compactness of a substance.".to_string(),
            related_concepts: vec!["Mass".to_string(), "Volume".to_string()],
        },
        VocabularyTerm {
            word: "Volume".to_string(),
            definition: "The amount of space that a substance or object occupies, or that is enclosed within a container.".to_string(),
            related_concepts: vec!["Space".to_string(), "Capacity".to_string()],
        },
        VocabularyTerm {
            word: "Pressure".to_string(),
            definition: "Continuous physical force exerted on or against an object by something in contact with it.".to_string(),
            related_concepts: vec!["Force".to_string(), "Area".to_string(), "Pascal".to_string()],
        },
        VocabularyTerm {
            word: "Temperature".to_string(),
            definition: "The degree or intensity of heat present in a substance or object.".to_string(),
            related_concepts: vec!["Heat".to_string(), "Energy".to_string(), "Thermometer".to_string()],
        },
        VocabularyTerm {
            word: "Heat".to_string(),
            definition: "Energy that is transferred from one body to another as the result of a difference in temperature.".to_string(),
            related_concepts: vec!["Energy".to_string(), "Transfer".to_string()],
        },
        VocabularyTerm {
            word: "Conduction".to_string(),
            definition: "The process by which heat or electricity is directly transmitted through a substance when there is a difference of temperature or of electrical potential between adjoining regions, without movement of the material.".to_string(),
            related_concepts: vec!["Heat".to_string(), "Transfer".to_string(), "Contact".to_string()],
        },
        VocabularyTerm {
            word: "Convection".to_string(),
            definition: "The movement caused within a fluid by the tendency of hotter and therefore less dense material to rise, and colder, denser material to sink under the influence of gravity, which consequently results in transfer of heat.".to_string(),
            related_concepts: vec!["Fluid".to_string(), "Heat".to_string(), "Movement".to_string()],
        },
        VocabularyTerm {
            word: "Radiation".to_string(),
            definition: "The emission of energy as electromagnetic waves or as moving subatomic particles, especially high-energy particles which cause ionization.".to_string(),
            related_concepts: vec!["Waves".to_string(), "Energy".to_string(), "Emission".to_string()],
        },
        VocabularyTerm {
            word: "Wave".to_string(),
            definition: "A long body of water curling into an arched form and breaking on the shore.".to_string(),
            related_concepts: vec!["Oscillation".to_string(), "Frequency".to_string(), "Amplitude".to_string()],
        },
        VocabularyTerm {
            word: "Frequency".to_string(),
            definition: "The rate at which something occurs or is repeated over a particular period of time or in a given sample.".to_string(),
            related_concepts: vec!["Hertz".to_string(), "Cycle".to_string(), "Period".to_string()],
        },
        VocabularyTerm {
            word: "Wavelength".to_string(),
            definition: "The distance between successive crests of a wave, especially points in a sound wave or electromagnetic wave.".to_string(),
            related_concepts: vec!["Distance".to_string(), "Wave".to_string()],
        },
        VocabularyTerm {
            word: "Amplitude".to_string(),
            definition: "The maximum extent of a vibration or oscillation, measured from the position of equilibrium.".to_string(),
            related_concepts: vec!["Height".to_string(), "Intensity".to_string()],
        },
        VocabularyTerm {
            word: "Reflection".to_string(),
            definition: "The throwing back by a body or surface of light, heat, or sound without absorbing it.".to_string(),
            related_concepts: vec!["Bounce".to_string(), "Mirror".to_string()],
        },
        VocabularyTerm {
            word: "Refraction".to_string(),
            definition: "The fact or phenomenon of light, radio waves, etc. being deflected in passing obliquely through the interface between one medium and another or through a medium of varying density.".to_string(),
            related_concepts: vec!["Bending".to_string(), "Lens".to_string()],
        },
        VocabularyTerm {
            word: "Diffraction".to_string(),
            definition: "The process by which a beam of light or other system of waves is spread out as a result of passing through a narrow aperture or across an edge, typically accompanied by interference between the wave forms produced.".to_string(),
            related_concepts: vec!["Spreading".to_string(), "Interference".to_string()],
        },
        VocabularyTerm {
            word: "Interference".to_string(),
            definition: "The combination of two or more electromagnetic waveforms to form a resultant wave in which the displacement is either reinforced or canceled.".to_string(),
            related_concepts: vec!["Superposition".to_string(), "Constructive".to_string(), "Destructive".to_string()],
        },
        VocabularyTerm {
            word: "Electricity".to_string(),
            definition: "A form of energy resulting from the existence of charged particles (such as electrons or protons), either statically as an accumulation of charge or dynamically as a current.".to_string(),
            related_concepts: vec!["Charge".to_string(), "Current".to_string(), "Energy".to_string()],
        },
        VocabularyTerm {
            word: "Current".to_string(),
            definition: "A flow of electricity which results from the ordered directional movement of electrically charged particles.".to_string(),
            related_concepts: vec!["Flow".to_string(), "Amperes".to_string()],
        },
        VocabularyTerm {
            word: "Voltage".to_string(),
            definition: "An electromotive force or potential difference expressed in volts.".to_string(),
            related_concepts: vec!["Potential".to_string(), "Force".to_string()],
        },
        VocabularyTerm {
            word: "Resistance".to_string(),
            definition: "The refusal to accept or comply with something; the attempt to prevent something by action or argument.".to_string(),
            related_concepts: vec!["Ohm".to_string(), "Impedance".to_string()],
        },
        VocabularyTerm {
            word: "Circuit".to_string(),
            definition: "A roughly circular line, route, or movement that starts and finishes at the same place.".to_string(),
            related_concepts: vec!["Loop".to_string(), "Path".to_string()],
        },
        VocabularyTerm {
            word: "Magnetism".to_string(),
            definition: "A physical phenomenon produced by the motion of electric charge, resulting in attractive and repulsive forces between objects.".to_string(),
            related_concepts: vec!["Field".to_string(), "Attraction".to_string(), "Repulsion".to_string()],
        },
        VocabularyTerm {
            word: "Atom".to_string(),
            definition: "The basic unit of a chemical element.".to_string(),
            related_concepts: vec!["Nucleus".to_string(), "Electron".to_string(), "Proton".to_string()],
        },
        VocabularyTerm {
            word: "Molecule".to_string(),
            definition: "A group of atoms bonded together, representing the smallest fundamental unit of a chemical compound that can take part in a chemical reaction.".to_string(),
            related_concepts: vec!["Bond".to_string(), "Compound".to_string()],
        },
        VocabularyTerm {
            word: "Proton".to_string(),
            definition: "A stable subatomic particle occurring in all atomic nuclei, with a positive electric charge equal in magnitude to that of an electron, but of much greater mass.".to_string(),
            related_concepts: vec!["Positive".to_string(), "Nucleus".to_string()],
        },
        VocabularyTerm {
            word: "Neutron".to_string(),
            definition: "A subatomic particle of about the same mass as a proton but without an electric charge, present in all atomic nuclei except those of ordinary hydrogen.".to_string(),
            related_concepts: vec!["Neutral".to_string(), "Nucleus".to_string()],
        },
        VocabularyTerm {
            word: "Electron".to_string(),
            definition: "A stable subatomic particle with a charge of negative electricity, found in all atoms and acting as the primary carrier of electricity in solids.".to_string(),
            related_concepts: vec!["Negative".to_string(), "Orbit".to_string()],
        },
        VocabularyTerm {
            word: "Nucleus".to_string(),
            definition: "The central and most important part of an object, movement, or group, forming the basis for its activity and growth.".to_string(),
            related_concepts: vec!["Center".to_string(), "Core".to_string()],
        },
        VocabularyTerm {
            word: "Isotope".to_string(),
            definition: "Each of two or more forms of the same element that contain equal numbers of protons but different numbers of neutrons in their nuclei, and hence differ in relative atomic mass but not in chemical properties; in particular, a radioactive form of an element.".to_string(),
            related_concepts: vec!["Element".to_string(), "Variation".to_string()],
        },
        VocabularyTerm {
            word: "Radioactivity".to_string(),
            definition: "The emission of ionizing radiation or particles caused by the spontaneous disintegration of atomic nuclei.".to_string(),
            related_concepts: vec!["Decay".to_string(), "Emission".to_string()],
        },
        VocabularyTerm {
            word: "Fission".to_string(),
            definition: "The action of dividing or splitting something into two or more parts.".to_string(),
            related_concepts: vec!["Splitting".to_string(), "Energy".to_string()],
        },
        VocabularyTerm {
            word: "Fusion".to_string(),
            definition: "The process or result of joining two or more things together to form a single entity.".to_string(),
            related_concepts: vec!["Joining".to_string(), "Energy".to_string(), "Sun".to_string()],
        },
        VocabularyTerm {
            word: "Relativity".to_string(),
            definition: "The absence of standards of absolute and universal application.".to_string(),
            related_concepts: vec!["Einstein".to_string(), "Time".to_string(), "Space".to_string()],
        },
        VocabularyTerm {
            word: "Quantum".to_string(),
            definition: "A discrete quantity of energy proportional in magnitude to the frequency of the radiation it represents.".to_string(),
            related_concepts: vec!["Discrete".to_string(), "Mechanics".to_string()],
        },
        VocabularyTerm {
            word: "Photon".to_string(),
            definition: "A particle representing a quantum of light or other electromagnetic radiation. A photon carries energy proportional to the radiation frequency but has zero rest mass.".to_string(),
            related_concepts: vec!["Light".to_string(), "Particle".to_string()],
        },
        VocabularyTerm {
            word: "Optics".to_string(),
            definition: "The scientific study of sight and the behavior of light, or the properties of transmission and deflection of other forms of radiation.".to_string(),
            related_concepts: vec!["Light".to_string(), "Vision".to_string()],
        },
        VocabularyTerm {
            word: "Acoustics".to_string(),
            definition: "The properties or qualities of a room or building that determine how sound is transmitted in it.".to_string(),
            related_concepts: vec!["Sound".to_string(), "Hearing".to_string()],
        },
    ]
}
