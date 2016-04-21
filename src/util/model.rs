use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

pub struct Model {
    vertices: Vec<Vector3f>,
    faces: Vec<Vec<i32>>
}

pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Model {

    pub fn new(filename: &str) -> Model
    {
        let mut faces = Vec::new();
        let mut vertices = Vec::new();

        // Open model
        let content = read_file(filename);
        let split = content.split("\n");

        // Parse file
        for s in split.clone() {
            let elements = s.split(" ");
            let vec: Vec<&str> = elements.collect();

            if vec[0] == "v" {
                // v -0.656078 -0.718512 -0.109025
                vertices.push(Vector3f {
                    x: vec[1].parse::<f64>().unwrap(),
                    y: vec[2].parse::<f64>().unwrap(),
                    z: vec[3].parse::<f64>().unwrap()
                });
            }

            if vec[0] == "f" {
                // f 117/98/117 378/363/378 255/236/255
                let mut face = Vec::<i32>::new();
                for i in 1..4 {
                    let face_elements = vec[i].split("/");
                    let face_vector: Vec<&str> = face_elements.collect();

                    // println!("{} / {} / {}", i32::from_str(face_vector[0]).unwrap() - 1, i32::from_str(face_vector[1]).unwrap() - 1, i32::from_str(face_vector[2]).unwrap() - 1);

                    //for j in 0..3 {
                        face.push(i32::from_str(face_vector[0]).unwrap() - 1);
                    //}
                }
                //println!("end");
                faces.push(face);
            }
        }

        println!("vectices: {}, faces: {}", vertices.len(), faces.len());

        Model {
            vertices: vertices,
            faces: faces
        }
    }

    pub fn nverts(&self) -> usize
    {
        self.vertices.len()
    }

    pub fn nfaces(&self) -> usize
    {
        self.faces.len()
    }

    pub fn face(&self, id: usize) -> &Vec<i32>
    {
        &self.faces[id]
    }

    pub fn vert(&self, id: usize) -> &Vector3f
    {
        &self.vertices[id]
    }
}

fn read_file(filename: &str) -> String
{
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => s
    }
}
