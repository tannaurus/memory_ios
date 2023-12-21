//
//  Prompt.swift
//  Memory
//
//  Created by Tanner on 11/25/23.
//

import Foundation

struct Prompt: Codable, Identifiable {
    let id = UUID()
    
    var name: String;
    var description: String;
    
    private enum CodingKeys: String, CodingKey {
        case name
        case description
    }
}
