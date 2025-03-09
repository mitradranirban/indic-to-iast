// src/lib.rs
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Export functions to JavaScript
#[wasm_bindgen]
pub fn transliterate(text: &str, script: &str) -> String {
    match script {
        "devanagari" => transliterate_devanagari(text),
        "bengali" => transliterate_bengali(text),
        "tamil" => transliterate_tamil(text),
        "telugu" => transliterate_telugu(text),
        "kannada" => transliterate_kannada(text),
        "malayalam" => transliterate_malayalam(text),
        "gujarati" => transliterate_gujarati(text),
        "odia" => transliterate_odia(text),
        "punjabi" => transliterate_punjabi(text),
        _ => String::from("Unsupported script")
    }
}

// Devanagari to IAST transliteration
fn transliterate_devanagari(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Vowels
    mapping.insert('अ', "a"); mapping.insert('आ', "ā");
    mapping.insert('इ', "i"); mapping.insert('ई', "ī");
    mapping.insert('उ', "u"); mapping.insert('ऊ', "ū");
    mapping.insert('ऋ', "ṛ"); mapping.insert('ॠ', "ṝ");
    mapping.insert('ऌ', "ḷ"); mapping.insert('ॡ', "ḹ");
    mapping.insert('ए', "e"); mapping.insert('ऐ', "ai");
    mapping.insert('ओ', "o"); mapping.insert('औ', "au");
    
    // Vowel Marks
    mapping.insert('ा', "ā"); mapping.insert('ि', "i");
    mapping.insert('ी', "ī"); mapping.insert('ु', "u");
    mapping.insert('ू', "ū"); mapping.insert('ृ', "ṛ");
    mapping.insert('ॄ', "ṝ"); mapping.insert('ॢ', "ḷ");
    mapping.insert('ॣ', "ḹ"); mapping.insert('े', "e");
    mapping.insert('ै', "ai"); mapping.insert('ो', "o");
    mapping.insert('ौ', "au"); mapping.insert('ं', "ṃ");
    mapping.insert('ः', "ḥ"); mapping.insert('ँ', "m̐");
    
    // Consonants
    mapping.insert('क', "ka"); mapping.insert('ख', "kha");
    mapping.insert('ग', "ga"); mapping.insert('घ', "gha");
    mapping.insert('ङ', "ṅa"); mapping.insert('च', "ca");
    mapping.insert('छ', "cha"); mapping.insert('ज', "ja");
    mapping.insert('झ', "jha"); mapping.insert('ञ', "ña");
    mapping.insert('ट', "ṭa"); mapping.insert('ठ', "ṭha");
    mapping.insert('ड', "ḍa"); mapping.insert('ढ', "ḍha");
    mapping.insert('ण', "ṇa"); mapping.insert('त', "ta");
    mapping.insert('थ', "tha"); mapping.insert('द', "da");
    mapping.insert('ध', "dha"); mapping.insert('न', "na");
    mapping.insert('प', "pa"); mapping.insert('फ', "pha");
    mapping.insert('ब', "ba"); mapping.insert('भ', "bha");
    mapping.insert('म', "ma"); mapping.insert('य', "ya");
    mapping.insert('र', "ra"); mapping.insert('ल', "la");
    mapping.insert('व', "va"); mapping.insert('श', "śa");
    mapping.insert('ष', "ṣa"); mapping.insert('स', "sa");
    mapping.insert('ह', "ha"); 
    
    // Modify the consonants to handle the implicit 'a' sound
    // and handle conjuncts correctly
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let current_char = chars[i];
        
        // Check for special conjuncts
        if i + 1 < chars.len() {
            let next_char = chars[i + 1];
            let conjunct = format!("{}{}", current_char, next_char);
            
            if let Some(&trans) = mapping.get(&conjunct.chars().next().unwrap()) {
                result.push_str(trans);
                i += 2;
                continue;
            }
        }
        
        // Handle regular characters
        if let Some(&trans) = mapping.get(&current_char) {
            // If this is a consonant (ends with 'a') and the next character is a vowel mark
            // or is part of a conjunct, then remove the trailing 'a'
            if trans.ends_with('a') && i + 1 < chars.len() {
                let next_char = chars[i + 1];
                if mapping.contains_key(&next_char) && 
                   (mapping[&next_char].len() == 1 || next_char == '्') {
                    result.push_str(&trans[0..trans.len()-1]);
                } else {
                    result.push_str(trans);
                }
            } else {
                result.push_str(trans);
            }
        } else if current_char == '्' {
            // Handle virama (halant) by not adding anything (consonant without vowel)
        } else {
            // If character not in mapping, keep it as is
            result.push(current_char);
        }
        
        i += 1;
    }
    
    result
}

// Implementation for Bengali to IAST
fn transliterate_bengali(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Vowels
    mapping.insert('অ', "a"); mapping.insert('আ', "ā");
    mapping.insert('ই', "i"); mapping.insert('ঈ', "ī");
    mapping.insert('উ', "u"); mapping.insert('ঊ', "ū");
    mapping.insert('ঋ', "ṛ"); mapping.insert('ৠ', "ṝ");
    mapping.insert('ঌ', "ḷ"); mapping.insert('ৡ', "ḹ");
    mapping.insert('এ', "e"); mapping.insert('ঐ', "ai");
    mapping.insert('ও', "o"); mapping.insert('ঔ', "au");
    
    // Vowel Marks
    mapping.insert('া', "ā"); mapping.insert('ি', "i");
    mapping.insert('ী', "ī"); mapping.insert('ু', "u");
    mapping.insert('ূ', "ū"); mapping.insert('ৃ', "ṛ");
    mapping.insert('ৄ', "ṝ"); mapping.insert('ৢ', "ḷ");
    mapping.insert('ৣ', "ḹ"); mapping.insert('ে', "e");
    mapping.insert('ৈ', "ai"); mapping.insert('ো', "o");
    mapping.insert('ৌ', "au"); mapping.insert('ং', "ṃ");
    mapping.insert('ঃ', "ḥ"); mapping.insert('ঁ', "m̐");
    
    // Consonants
    mapping.insert('ক', "ka"); mapping.insert('খ', "kha");
    mapping.insert('গ', "ga"); mapping.insert('ঘ', "gha");
    mapping.insert('ঙ', "ṅa"); mapping.insert('চ', "ca");
    mapping.insert('ছ', "cha"); mapping.insert('জ', "ja");
    mapping.insert('ঝ', "jha"); mapping.insert('ঞ', "ña");
    mapping.insert('ট', "ṭa"); mapping.insert('ঠ', "ṭha");
    mapping.insert('ড', "ḍa"); mapping.insert('ঢ', "ḍha");
    mapping.insert('ণ', "ṇa"); mapping.insert('ত', "ta");
    mapping.insert('থ', "tha"); mapping.insert('দ', "da");
    mapping.insert('ধ', "dha"); mapping.insert('ন', "na");
    mapping.insert('প', "pa"); mapping.insert('ফ', "pha");
    mapping.insert('ব', "ba"); mapping.insert('ভ', "bha");
    mapping.insert('ম', "ma"); mapping.insert('য', "ya");
    mapping.insert('র', "ra"); mapping.insert('ল', "la");
    mapping.insert('শ', "śa"); mapping.insert('ষ', "ṣa");
    mapping.insert('স', "sa"); mapping.insert('হ', "ha");
    
    // Similar algorithm as devanagari but adapting for Bengali specifics
    // This is a simplified version - a full version would need more complex handling
    let mut result = String::new();
    for c in text.chars() {
        if let Some(&trans) = mapping.get(&c) {
            result.push_str(trans);
        } else {
            result.push(c);
        }
    }
    
    result
}

// Tamil to IAST implementation
fn transliterate_tamil(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Vowels
    mapping.insert('அ', "a"); mapping.insert('ஆ', "ā");
    mapping.insert('இ', "i"); mapping.insert('ஈ', "ī");
    mapping.insert('உ', "u"); mapping.insert('ஊ', "ū");
    mapping.insert('எ', "e"); mapping.insert('ஏ', "ē");
    mapping.insert('ஐ', "ai"); mapping.insert('ஒ', "o");
    mapping.insert('ஓ', "ō"); mapping.insert('ஔ', "au");
    
    // Consonants (with implicit 'a')
    mapping.insert('க', "ka"); mapping.insert('ங', "ṅa");
    mapping.insert('ச', "ca"); mapping.insert('ஞ', "ña");
    mapping.insert('ட', "ṭa"); mapping.insert('ண', "ṇa");
    mapping.insert('த', "ta"); mapping.insert('ந', "na");
    mapping.insert('ப', "pa"); mapping.insert('ம', "ma");
    mapping.insert('ய', "ya"); mapping.insert('ர', "ra");
    mapping.insert('ல', "la"); mapping.insert('வ', "va");
    mapping.insert('ழ', "ḻa"); mapping.insert('ள', "ḷa");
    mapping.insert('ற', "ṟa"); mapping.insert('ன', "ṉa");
    
    // Simplified handling - similar structure as previous functions
    let mut result = String::new();
    for c in text.chars() {
        if let Some(&trans) = mapping.get(&c) {
            result.push_str(trans);
        } else {
            result.push(c);
        }
    }
    
    result
}

// Telugu to IAST implementation
fn transliterate_telugu(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Telugu to IAST not fully implemented yet")
}

// Kannada to IAST implementation
fn transliterate_kannada(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Kannada to IAST not fully implemented yet")
}

// Malayalam to IAST implementation
fn transliterate_malayalam(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Malayalam to IAST not fully implemented yet")
}

// Gujarati to IAST implementation
fn transliterate_gujarati(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Gujarati to IAST not fully implemented yet")
}

// Odia to IAST implementation
fn transliterate_odia(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Odia to IAST not fully implemented yet")
}

// Punjabi (Gurmukhi) to IAST implementation
fn transliterate_punjabi(_text: &str) -> String {
    // Similar implementation as previous functions
    String::from("Punjabi to IAST not fully implemented yet")
}

// Add a function to list supported scripts
#[wasm_bindgen]
pub fn supported_scripts() -> String {
    String::from("devanagari,bengali,tamil,telugu,kannada,malayalam,gujarati,odia,punjabi")
}

// For logging to browser console from Rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Helper function to log messages to browser console
#[wasm_bindgen]
pub fn console_log(msg: &str) {
    log(msg);
}

