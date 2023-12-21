//
//  Profile.swift
//  Memory
//
//  Created by Tanner on 11/25/23.
//

import Foundation

struct User: Codable, Identifiable {
    let id = UUID()
    
    var name: String;
    var picture: String;
    var followers: Int32;
    var following: Int32;
    var bio: String;
    
    
    private enum CodingKeys: String, CodingKey {
        case name
        case picture
        case followers
        case following
        case bio
    }
}
