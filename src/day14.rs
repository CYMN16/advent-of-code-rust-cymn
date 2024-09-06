use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct Ingredient {
    amount: i64,
    name: String,
}

impl Ingredient {
    fn new(str: &str) -> Self {
        let description: Vec<_> = str.trim().split_terminator(" ").collect();
        let amount = description[0].trim().parse::<i64>().unwrap();
        let name = description[1].trim().to_string();
        Self { amount, name }
    }
}

#[derive(Clone, Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
    output: Ingredient,
}

impl Recipe {
    fn new(str: &str) -> Self {
        // 1 HKCVW, 2 DFCT => 5 ZJZRN
        // "1 HKCVW", " 2 DFCT ",  "=>" , " 5 ZJZRN"
        let token_vec: Vec<_> = str.split_terminator("=>").collect();
        let mut ingredient_vec = vec![];
        let ingredients: Vec<_> = token_vec[0].split_terminator(",").collect();
        for n in ingredients {
            ingredient_vec.push(Ingredient::new(n));
        }

        let output = Ingredient::new(token_vec[1]);
        Self { ingredients: ingredient_vec, output }
    }
}
#[derive(Clone, Debug)]
pub struct RecipeBook {
    recipes: HashMap<String, Recipe>,
}

impl RecipeBook {
    pub fn new(str: &str) -> Self {
        let mut output = Self { recipes: HashMap::new() };
        let recipe_vec: Vec<_> = str.split_terminator("\n").collect();
        for x in recipe_vec {
            let recipe = Recipe::new(x);
            output.recipes.insert(recipe.output.name.to_string(), recipe);
        }
        output
    }

    // pub fn calculate_required_ore_for_recipe(&self, output: &str, amount: i64) -> i64 {
    //     let mut ore_map: HashMap<String, i64> = HashMap::new();
    //     let mut process: VecDeque<(String, i64)> = VecDeque::new();
    //     process.push_back((output.to_string(), amount));
    //
    //     let mut total_ore = 0;
    //
    //     loop {
    //         let (current_ingredient, current_amount) = process.pop_front().unwrap();
    //         if current_ingredient != "ORE" {
    //             let recipe = self.recipes.get(&current_ingredient).unwrap();
    //             let recipe_amount = recipe.output.amount;
    //
    //             let times = if current_amount % recipe_amount != 0 {
    //                 current_amount / recipe_amount + 1
    //             } else {
    //                 current_amount / recipe_amount
    //             };
    //
    //             for ingredient in recipe.ingredients.clone() {
    //                 process.push_back((ingredient.name, ingredient.amount * times));
    //             }
    //         } else {
    //             total_ore += current_amount;
    //         }
    //         if process.is_empty() { break; };
    //     }
    //
    //
    //     // println!("{:?}", ore_map);
    //     total_ore
    // }


    // pub fn calculate_required_ore_for_recipe(&self, output: &str, amount: i64) -> i64 {
    //     let mut ore_map: HashMap<String, i64> = HashMap::new();
    //     let mut process: VecDeque<(String, i64)> = VecDeque::new();
    //     process.push_back((output.to_string(), amount));
    //     loop {
    //         let (current_ingredient, current_amount) = process.pop_front().unwrap();
    //         let recipe = self.recipes.get(&current_ingredient).unwrap();
    //         let recipe_amount = recipe.output.amount;
    //
    //         let times = if current_amount % recipe_amount != 0 {
    //             current_amount / recipe_amount + 1
    //         } else {
    //             current_amount / recipe_amount
    //         };
    //
    //         for ingredient in recipe.ingredients.clone() {
    //             if ingredient.name == "ORE" {
    //                 *ore_map.entry(current_ingredient.clone()).or_insert(0) += current_amount;
    //                 continue;
    //             };
    //             process.push_back((ingredient.name, ingredient.amount * times));
    //         }
    //         if process.is_empty() { break; };
    //     }
    //
    //     let mut min_ore = 0;
    //
    //     for (key, amount) in ore_map {
    //         let recipe = self.recipes.get(&key).unwrap();
    //         let recipe_amount = recipe.output.amount;
    //         let times = if amount % recipe_amount != 0 {
    //             amount / recipe_amount + 1
    //         } else {
    //             amount / recipe_amount
    //         };
    //
    //         min_ore += times * recipe.ingredients[0].amount;
    //
    //         // println!("Recipe: {recipe:?}, times: {times}, min_ore: {min_ore})");
    //     }
    //
    //     // println!("{:?}", ore_map);
    //     min_ore
    // }
}

#[derive(Clone, Debug)]
pub struct Inventory {
    remaining_ingredient_map: HashMap<String, i64>,
    recipe_book: RecipeBook,
    total_used_ore: i64,
}

impl Inventory {
    //Idea: craft the items only when necessary to minimize ore consumption, check the inventory for the required items
    pub fn new(recipe_book: RecipeBook) -> Self {
        Self { remaining_ingredient_map: HashMap::new(), recipe_book, total_used_ore: 0 }
    }

    pub fn request_item(&mut self, item: &str, amount: i64) {
        let req_item_count = *self.remaining_ingredient_map.entry(item.to_string()).or_insert(0);
        if req_item_count < amount {
            let current_recipe = self.recipe_book.recipes.get(item).unwrap().clone();
            if current_recipe.ingredients.len() == 1 && current_recipe.ingredients[0].name == "ORE" {
                let ingredient_count = *self.remaining_ingredient_map.entry(item.to_string()).or_insert(0);
                if ingredient_count < amount {
                    self.craft_item(item, amount - ingredient_count);
                }
            } else {
                let recipe_amount = current_recipe.output.amount;
                let req_amount = amount - req_item_count;
                let n = if req_amount % recipe_amount != 0 {// how many times to use the recipe at once
                    req_amount / recipe_amount + 1
                } else {
                    req_amount / recipe_amount
                };
                for ingredient in current_recipe.ingredients.clone() {
                    self.request_item(ingredient.name.as_str(), ingredient.amount * n);
                    *self.remaining_ingredient_map.get_mut(ingredient.name.as_str()).unwrap() -= ingredient.amount * n;
                }
                // there are enough ingredients to craft the recipe
                *self.remaining_ingredient_map.get_mut(item).unwrap() += current_recipe.output.amount * n;
            }
        }
        // *self.remaining_ingredient_map.entry(item.to_string()).or_insert(0) -= amount;
    }

    fn craft_item(&mut self, item: &str, amount: i64) {
        let current_recipe = self.recipe_book.recipes.get(item).unwrap();
        let recipe_amount = current_recipe.output.amount;
        let n = if amount % recipe_amount != 0 {
            amount / recipe_amount + 1
        } else {
            amount / recipe_amount
        };
        self.total_used_ore += current_recipe.ingredients[0].amount * n;
        *self.remaining_ingredient_map.entry(item.to_string()).or_insert(0) += current_recipe.output.amount * n;
    }
    
    pub fn craft_until_n(&mut self, item: &str, n: i64) -> i64 {
        let mut temp_self = self.clone();
        let mut multiplier = 1000000;
        let mut produced_fuel = 0;
        loop {
            if multiplier < 1 {break};
             if self.get_used_ore() > n {
                produced_fuel -= multiplier;
                multiplier /= 2;
                *self = temp_self.clone();
            } else{
                temp_self = self.clone();
                self.request_item(item, multiplier);
                *self.remaining_ingredient_map.get_mut("FUEL").unwrap() -= multiplier;
                produced_fuel += multiplier;
            }
        }
        produced_fuel
    }

    pub fn get_used_ore(&self) -> i64 {
        self.total_used_ore
    }
    pub fn get_unused_ingredients(&self) -> HashMap<String, i64> {
        self.remaining_ingredient_map.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_recipe_1() -> String {
        "1 HKCVW, 2 DFCT, 2 ASD, 55 DASODAD => 5 ZJZRN".to_string()
    }
    fn example_recipe_2() -> String {
        "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL".to_string()
    }
    fn example_recipe_3() -> String {
        "157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string()
    }
    fn example_recipe_4() -> String {
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF".to_string()
    }

    fn example_recipe_5() -> String {
        "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX".to_string()
    }
    #[test]
    fn test_parse_recipe() {
        let recipe = Recipe::new(example_recipe_1().as_str());

        println!("{:?}", recipe);
    }

    #[test]
    fn test_parse_recipe_book() {
        let recipe_book = RecipeBook::new(example_recipe_2().as_str());

        println!("{:?}", recipe_book);
    }

    #[test]
    fn test_calculate_required_ore_for_recipe() {
        // let recipe_book = RecipeBook::new(example_recipe_2().as_str());
        //
        // println!("{}", recipe_book.calculate_required_ore_for_recipe("FUEL", 1));
        // let recipe_book = RecipeBook::new(example_recipe_3().as_str());
        //
        // println!("{}", recipe_book.calculate_required_ore_for_recipe("FUEL", 1));
        // let recipe_book = RecipeBook::new(example_recipe_4().as_str());
        //
        // println!("{}", recipe_book.calculate_required_ore_for_recipe("FUEL", 1));
        // let recipe_book = RecipeBook::new(example_recipe_5().as_str());
        //
        // println!("{}", recipe_book.calculate_required_ore_for_recipe("FUEL", 1));
    }

    #[test]
    fn test_inventory() {
        let recipe_book = RecipeBook::new(example_recipe_5().as_str());
        let mut inventory = Inventory::new(recipe_book);

        println!("Before {:?}", inventory.remaining_ingredient_map);
        inventory.request_item("FUEL", 1);
        println!("After {:?}", inventory.remaining_ingredient_map);
        println!("total ore: {:?}", inventory.total_used_ore);
    }

    #[test]
    fn test_max_fuel() {
        let n: i64 = 1_000_000_000_000;
        let recipe_book = RecipeBook::new(example_recipe_5().as_str());
        let mut inventory = Inventory::new(recipe_book);

        println!("{}", inventory.craft_until_n("FUEL", n));
        
    }
}