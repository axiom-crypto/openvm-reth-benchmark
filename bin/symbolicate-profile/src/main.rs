use clap::Parser;
use serde_json::{from_str, to_string_pretty, Value};
use std::{collections::HashMap, fs, path::Path};
use wholesym::{FramesLookupResult, SymbolManager, SymbolManagerConfig};

#[derive(Parser, Debug)]
#[command(author, version, about = "Symbolicate a Samply profile.json using a dSYM file", long_about = None)]
struct Args {
    /// Path to the unsymbolicated profile.json file
    #[arg(short, long, default_value = "profile.json")]
    input: String,

    /// Path to the symbols file
    #[arg(short, long, help = "Path to the symbols file (.dSYM, .debug, .pdb)")]
    symbols: String,

    /// Path to save the symbolicated profile.json file
    #[arg(short, long, default_value = "profile_symbolicated.json")]
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();

    // Configure the SymbolManager with the symbols file
    let symbol_manager = SymbolManager::with_config(SymbolManagerConfig::default());
    let symbol_map =
        symbol_manager.load_symbol_map_for_binary_at_path(Path::new(&args.symbols), None).await?;

    // Load the unsymbolicated profile.json
    let profile_json = fs::read_to_string(&args.input)?;
    let mut profile: Value = from_str(&profile_json)?;

    // Extract threads and update frames
    if let Some(threads) = profile.get_mut("threads").and_then(|t| t.as_array_mut()) {
        for thread in threads {
            let mut thread_clone = thread.clone();
            let frame_table = thread_clone
                .get_mut("frameTable")
                .and_then(|f| f.as_object_mut())
                .ok_or("No frameTable")?;
            // let string_table = thread
            //     .get_mut("stringTable")
            //     .and_then(|s| s.as_array_mut())
            //     .ok_or("No stringTable")?;

            // Collect addresses from frameTable
            let mut address_map: HashMap<usize, u64> = HashMap::new();
            if let Some(locations) = frame_table.get("address").and_then(|l| l.as_array()) {
                for (i, loc) in locations.iter().enumerate() {
                    if let Some(addr) = loc.as_u64() {
                        address_map.insert(i, addr);
                    }
                }
            }

            // Symbolicate addresses
            let addresses: Vec<u64> = address_map.values().copied().collect();
            println!("addresses: {:?}", addresses);
            for addr in addresses {
                let symbolicated_frames = symbol_map.lookup(addr).await?;
                println!("symbolicated_frames: {:?}", symbolicated_frames);

                // Update stringTable and frameTable
                let mut string_index_map: HashMap<String, usize> = HashMap::new();
                let mut next_string_index = string_table.len();

                for (frame_idx, addr) in address_map.iter() {
                    let symbol_frame = &symbolicated_frames[*frame_idx];
                    let symbol_info = format_symbol_info(&symbol_frame);

                    // Add to stringTable if not already present
                    let string_idx = match string_index_map.get(&symbol_info) {
                        Some(&idx) => idx,
                        None => {
                            string_table.push(Value::String(symbol_info.clone()));
                            string_index_map.insert(symbol_info.clone(), next_string_index);
                            next_string_index += 1;
                            next_string_index - 1
                        }
                    };

                    // Update frameTable location with the new string index
                    if let Some(locations) =
                        frame_table.get_mut("location").and_then(|l| l.as_array_mut())
                    {
                        locations[*frame_idx] = Value::Number(string_idx.into());
                    }
                }
            }
        }
    }

    // Save the symbolicated profile
    fs::write(&args.output, to_string_pretty(&profile)?)?;
    println!("Symbolicated profile saved to {}", args.output);

    Ok(())
}

// Helper function to format symbol information
// fn format_symbol_info(frame: &FramesLookupResult) -> String {
//     match &frame {
//         FramesLookupResult::Available(info) => {
//             let mut result = String::new();
//             for frame in info {
//                 let func_name = frame.function.as_deref().unwrap_or("<unknown>");
//                 let file_name = frame
//                     .file_path
//                     .as_ref()
//                     .and_then(|p| p.file_name().and_then(|n| n.to_str()))
//                     .unwrap_or("<unknown>");
//                 let line = frame.line_number.unwrap_or(0);
//                 result.push_str(&format!("{} ({}:{})\n", func_name, file_name, line));
//             }
//             result
//         }
//         FramesLookupResult::External(addr) => format!("<unknown> [{:?}]", addr.address_in_file),
//     }
// }
