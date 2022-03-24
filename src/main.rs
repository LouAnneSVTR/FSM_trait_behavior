pub type NodeIndex = usize;
pub type TransitionIndex = usize;

/*
pub trait FSM {
    fn new(nameFSM: &str) -> SimpleFiniteStateMachine;
    fn addNode(&mut self, name: i32) -> NodeIndex;
    fn removeNode(&mut self, name: i32) -> NodeIndex;
    fn addTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex;
    fn removeTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex;
    fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool;
    fn displayTransition(&self);
    fn displayFSM(&self);
    //fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)>;
    fn print_vec( vec : Vec<Vec<&str>>);
}

pub struct SimpleFiniteStateMachine {
    name_fsm: String,
    transitions:  Box<T>::new<Vec<dyn Transition>>,
    nodes: Vec<dyn Node>
}*/

pub trait NamedElement{
    fn get_name(self) -> String;
}

pub trait Node : NamedElement {
    fn new(nameNode: &str,output_transition: Vec<TransitionIndex>,input_transition: Vec<TransitionIndex> ) -> NodeImpl;
    fn get_name(self) -> String;
    fn get_node(self) -> NodeImpl;
}
pub struct NodeImpl {
    name_node: String,
    output_transition: Vec<TransitionIndex>,
    input_transition: Vec<TransitionIndex>
}

impl NamedElement for NodeImpl {
    fn get_name(self) -> String {
        self.name_node
    }


}

impl Node for NodeImpl {
    fn new(nameNode: &str, output_transition: Vec<TransitionIndex>, input_transition: Vec<TransitionIndex>) -> NodeImpl {
        todo!()
    }

    fn get_name(self) -> String {
        todo!()
    }

    fn get_node(self) -> NodeImpl {
        todo!()
    }
}

pub trait Transition : NamedElement {
    fn new(name_transition: &str,letter: char,output_nodes: NodeIndex, input_nodes: NodeIndex) -> TransitionImpl;
    fn get_name(self) -> String;
    fn get_node(self) -> TransitionImpl;

}
pub struct TransitionImpl {
    pub name_transition: String,
    pub letter: char,
    output_nodes: NodeIndex,
    input_nodes: NodeIndex,
}

impl NamedElement for TransitionImpl {
    fn get_name(self) -> String {
        self.name_transition
    }
}

impl Transition for TransitionImpl {
    fn new(name_transition: &str, letter: char, output_nodes: NodeIndex, input_nodes: NodeIndex) -> TransitionImpl {
        todo!()
    }

    fn get_name(self) -> String {
        todo!()
    }

    fn get_node(self) -> TransitionImpl {
        todo!()
    }
}
/*
impl FSM for SimpleFiniteStateMachine {
    fn new(name_fsm: &str) -> SimpleFiniteStateMachine {
        return SimpleFiniteStateMachine {
            name_fsm: name_fsm.into(),
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    fn addNode(&mut self, name: i32) -> NodeIndex {
        let index: usize = self.nodes.len();
        self.nodes.push(NodeImpl {
            name,
            output_transition: Vec::new(),
            input_transition: Vec::new(),
        });
        return index;
    }

    fn addTransition(&mut self, letter: char, input_nodes: NodeIndex, output_nodes: NodeIndex) -> TransitionIndex {
        let index: usize = self.transitions.len();
        let new_trans = Transition {
            letter,
            output_nodes,
            input_nodes,
        };
        self.transitions.push(new_trans);

        self.nodes[input_nodes].output_transition.push(index);
        self.nodes[output_nodes].input_transition.push(index);
        return index;
    }


    fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {
        return self.transitions[transitionTest].letter == c_char;
    }

    fn displayTransition(&self) {
        println!("\n");
        for transitionNumber in &self.transitions {
            println!("{} {} {} {} {}", self.nodes[transitionNumber.input_nodes].name, "-", transitionNumber.letter, "->", self.nodes[transitionNumber.output_nodes].name);
        }
    }

    fn displayFSM(&self) {
        println!("{}", "\nDisplay Finite state machine : \n");
        print!("State : ");

        for nodeNumber in &self.nodes {
            print!("{} {}", nodeNumber.name, " ");
        }

        self.displayTransition()
    }


    fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)> {
        let mut vec_edge = Vec::new();

        let octets = word.as_bytes();

        let mut iterator_word = 0;
        let mut iterator_trans = 0;
        let mut state = 0;

        for (iterator_word, &char) in octets.iter().enumerate() {
            let char_trans = self.transitions[self.nodes[nodeIndex].output_transition[iterator_trans]].letter;

            while iterator_trans < self.nodes[nodeIndex].output_transition.len() && char_trans != char as char {
                iterator_trans += 1;
            }
            if self.transitions[iterator_trans].output_nodes < self.nodes.len() {
                vec_edge.push((state,self.transitions[iterator_trans].output_nodes));
                state = iterator_trans;
                iterator_trans = 0;
            }
        }

        if vec_edge.len() < word.len(){
            println!("Pas de chemin complet trouver pour ce mot.")
        } else {
            println!("Chemin trouvé pour ce mot.")
        }
        return vec_edge;

    }

    fn print_vec( vec : Vec<Vec<&str>>) {
        let state_nb = vec.len();

        for i in 0..state_nb {
            for j in 0..state_nb {
                print!("{}", vec[i][j]);
            }
            println!();
        }
        println!();
    }

    fn removeNode(&mut self, name: i32) -> NodeIndex {
        todo!()
    }

    fn removeTransition(&mut self, letter: char, input_nodes: NodeIndex, output_nodes: NodeIndex) -> TransitionIndex {
        todo!()
    }
}



fn main() {
/*
    let mut test = SimpleFiniteStateMachine::new("Test");
    let name = 0;
    let name2 = 1;
    let name3 = 2;

    test.addNode(name);
    test.addNode(name2);
    test.addNode(name3);

    test.addTransition('a',0,1);
    test.addTransition('b',0,2);
    test.addTransition('c',1,2);
    test.addTransition('d',2,2);

    test.displayFSM();

    let word1 = "acd";
    let mut result = test.process_fsm(0,word1);
    let lenght = result.len();
    print!("Taille du chemin pour ce mot.");
    print!("{}",lenght);
    dbg!(result);*/
} */

fn main() {}
