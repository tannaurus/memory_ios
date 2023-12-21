//
//  Story.swift
//  Memory
//
//  Created by Tanner on 11/24/23.
//

import Foundation

struct StoryImage: Codable, Identifiable {
    let id = UUID()
    
    var image: String;
    var description: String;
    
    private enum CodingKeys: String, CodingKey {
        case image
        case description
    }
}

struct Story: Codable, Identifiable {
    let id = UUID()
    
    var title: String;
    var preview: String;
    var images: [StoryImage];
    var created_at: String;
    var updated_at: String;
    
    private enum CodingKeys: String, CodingKey {
        case title
        case preview
        case images
        case created_at
        case updated_at
    }
}
