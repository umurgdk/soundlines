use cgmath::Vector3;
use three::Geometry;
use three::Material;
use three::Group;
use three::Factory;
use three::Mesh;

use dna::Dna;
use plant_setting::PlantSetting;
use cell::Cell;
use helpers::*;

pub struct Plant {
	pub dna: Dna,
	pub setting: PlantSetting,

	pub location: Vector3<f32>,
	pub cell_x: i32,
	pub cell_y: i32,

	pub size: f32,
	pub age: f32,
	pub fitness: f32,
	pub life_expectancy: f32,
	pub neighbor_count: i32,

	pub nickname: String,  // if a plant by users
	pub pitch: Vec<f32>,  //

	pub pre_mating_timer: f32,
	pub mating_timer: f32, // for mating period
	pub fruit_timer: f32,  
	pub index: i32,
	pub alpha: f32,

	pub overcrowded: bool,
	pub begin: bool,
	pub mating: bool,
	pub fruit: bool,

	pub mesh: Mesh
}

impl Plant {
    pub fn new(factory: &mut Factory, grp: &mut Group, setting: PlantSetting, dna: Dna, cell_x: i32, cell_y: i32, location: Vector3<f32>) -> Plant {
        let life_expectancy = dna.life_expectancy;
        let fitness = dna.fitness;
        let size = dna.size;

	    let mut mesh = factory.mesh(setting.geometry.clone(), Material::MeshBasic { color: 0x88aa22, map: None, wireframe: false });
	    let mesh_location: [f32; 3] = location.clone().into();
	    mesh.set_position(mesh_location);
	    mesh.set_scale(0.00001);

        let plant = Plant {
            dna,
            location,
            cell_x,
            cell_y,
            size,
            fitness,
            life_expectancy,
            mesh,
            age: 1.0,
            mating: false,
            pre_mating_timer: 1.0,
            fruit_timer: 0.0,
            index: 0,
            pitch: vec![],
            setting,
            nickname: "p".to_string(),
            mating_timer: 1.0,
            neighbor_count: 0,
            alpha: 255.0,
	        overcrowded: false,
	        begin: true,
	        fruit: false,
        };

	    grp.add(&plant.mesh);
	    plant
    }

    pub fn is_dead(&self) -> bool {
 		self.fitness < 0.0 || self.age > self.life_expectancy
    }

    pub fn update(&mut self, _: &mut Group, cell_wifi: f32, cell_sound: f32, cell_light: f32) {
  		// calculate Sensitivity for wifi, light, sound
  		let mut sensitivity = 1.0;

     	if cell_wifi > 0.5 {
       		//wifi sensitivity
       		let wifi =  
       			(map(self.setting.wifi_sensitivity, -1.0, 1.0, 0.2, 5.0) *
       		     map(cell_wifi, 0.0, 1.0, 0.5, 1.0)) / 2.6;

        	//light sensitivity
        	let light = 
        		(map(self.setting.light_sensitivity, -1.0, 1.0, 0.2, 5.0) *
        		 map(cell_light, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

          	//sound sensitivity
        	let sound = 
        		(map(self.setting.sound_sensitivity, -1.0, 1.0, 0.2, 5.0) *
        		 map(cell_sound, 0.0, 1.0, 0.5, 1.0) ) / 2.6;

        	sensitivity = wifi * light * sound;
     	} else if cell_wifi <= 0.5  {
       		//wifi sensitivity
       		let wifi = 
       			(map(self.setting.wifi_sensitivity , -1.0, 1.0, 5.0, 0.2) *
       			 map(cell_wifi, 0.0, 1.0, 0.5, 1.0) ) / 1.3  ;

        	//light sensitivity
       		let light = 
       			(map(self.setting.light_sensitivity, -1.0, 1.0, 5.0, 0.2) *
       			 map(cell_light, 0.0, 1.0, 0.5, 1.0) ) / 1.3;

        	//sound sensitivity
       		let sound = 
       			(map(self.setting.sound_sensitivity, -1.0, 1.0, 5.0, 0.2) *
       			 map(cell_sound, 0.0, 1.0, 0.5, 1.0) ) / 1.3 ;

        	sensitivity = wifi * light * sound;
     	}

  		//update age with sensitivity
  		self.age += self.dna.aging_rate / sensitivity;
	    let scale = self.age / (self.life_expectancy / 2.0);
	    self.mesh.set_scale(scale);
	    self.mesh.set_position([self.location.x, (10.0 / 2.0) * scale, self.location.z]);

  		//update growth
  		if self.fitness > 30.0 && self.size < self.setting.growth_limit {              // plant only grows when it is fit
     		self.size += self.dna.growth_rate * sensitivity;
  		}

  		// stressed when overcrowded 
  		if self.overcrowded { 
     		self.fitness -= (self.dna.stress_rate / sensitivity) * self.neighbor_count as f32 * 4.0;
  		}

  		//mating condition  
  		self.pre_mating_timer += self.dna.growth_rate * 3.0;   //the healthier the more chance of mating

    	if (self.pre_mating_timer % self.setting.mating_freq) as i32 == 0 && random(0.0, 1.0) < self.setting.birth_proba {   //probability for birth
      		if !self.mating && !self.fruit {
         		self.mating = true;
      		}
   		}

  		if self.mating {    // setup mating period the longer the more chance to give a birth
     		self.mating_timer += 0.01; 

     		if self.mating_timer > 15.0 {
        		self.mating = false;
        		self.mating_timer = 0.0;
   			}
  		}

   		if self.fruit {         
     		self.fruit_timer += 0.1;

     		if self.fruit_timer > 15.0 {
        		self.fruit = false;
        		self.fruit_timer = 0.0;
     		}
   		}

  		self.neighbor_count = 0;

        // TODO: Do that in the world
/*
 *  		for (int h = PcellX - 1; h <= PcellX + 1; h ++ ) {     //cells aroun this
 *       		for (int j = PcellY - 1; j <= PcellY + 1; j ++ ) {
 *                if( h >= 0 && j >= 0 ) {    // for the edge
 *                    if( h <= cellX && j <= cellY ) { 
 *                    	for (int i = 0;  i < world.cells[h][j].plantsList.size(); i++ ) {           
 *                         	Plant other = (Plant)world.cells[h][j].plantsList.get(i);
 *                         	float dist = dist(location.x, location.y, location.z,
 *                                    other.location.x, other.location.y, other.location.z);   //distance to partner
 *
 *                         	if(other != this && dist < dist_overcrowded) {  //check overcrowded      
 *                             	neighborCount++;
 *                           	}
 *
 *                          	if(mating == true) {   //mating
 *                             	if(other != this && other.mating == true ) {
 *                                 	if(random(0,1) < probaForBirth && dist < dist_mating) {    //probability
 *                                    	if(fruit == false) {
 *                                       		DNA child = dna.reproduce(other.dna);  //making DNA for child
 *
 *                                        	PVector child_location = new PVector(location.x + other.location.x + random(-windSpeed,windSpeed),
 *                                                    location.y + other.location.y,
 *                                                    location.z + other.location.z + random(-windSpeed,windSpeed));   // Seed spreads more when the wind speed is higher
 *                                        	child_location.mult(0.5); 
 *
 *                                       		// in case seed plant over the boundary
 *
 *                                       		if(child_location.x <= boundaryX1) {
 *                                         		child_location.x = boundaryX1  + 50;
 *                                       		}
 *
 *                                       		if(child_location.x >= boundaryX2) {
 *                                        		child_location.x = boundaryX2 - 50; 
 *                                       		}
 *
 *                                       		if(child_location.z <= boundaryY1) {
 *                                         		child_location.z = boundaryY1 + 50;
 *                                       		}
 *
 *                                       		if(child_location.z >= boundaryY2) {
 *                                        		child_location.z = boundaryY2 - 50; 
 *                                       		}
 *
 *                                        	float record = 10000;
 *                                        	int x = PcellX;
 *                                        	int y = PcellY;
 *
 *                                        	for (int k = PcellX - 1; k <= PcellX + 1; k ++ ) {
 *                                             	for (int l = PcellY - 1; l <= PcellY + 1; l ++ ) {
 *                                                  	if( k >= 0 && l >= 0 ) { 
	 *                                                     	if( k <= cellX && l <= cellY ) {
	 *                                                        	float dist2 = dist(child_location.x,
	 *                                                                    child_location.y,
	 *                                                                    child_location.z,
	 *
	 *                                                                    world.cells[k][l].location.x,
	 *                                                                    world.cells[k][l].location.y,
	 *                                                                    world.cells[k][l].location.z);
	 *
	 *                                                        	if(dist2 < record) {
	 *                                                           		record = dist2;
	 *                                                           		x = k;
	 *                                                           		y = l;
	 *                                                        	}
	 *                                                    	}
	 *                                                	}
	 *                                            	}
 *                                       		}
 *
 *                                       		//Producing seed
 *                                    		world.cells[x][y].seedList.add (  new Seed(child_location, child, x, y));
 *
 *                                    		mating = false;
 *                                    		matingTimer = 0;
 *                                    		other.mating = false;
 *                                    		other.matingTimer = 0;    
 *
 *                                    		fruit = true;
 *                                    		other.fruit = true;
 *
 *                              			}
 *
 *                          			}
 *
 *                      			}
 *
 *                  			}
 *
 *               			}
 *
 *            		}
 *
 *         		}
 *
 *      		}
 *
 *   		}
 *
 *  		//decide whether the area is overcrowded
 *    	if(neighborCount >= toleranceNeighbor_plant1) {
 *      		overcrowded = true;
 *    	} else if(neighborCount < toleranceNeighbor_plant1) {
 *     		overcrowded = false; 
 *    	}
 */
    }
}
