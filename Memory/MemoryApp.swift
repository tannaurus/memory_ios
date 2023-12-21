//
//  MemoryApp.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

@main
struct MemoryApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView(stories: mock_stories)
        }
    }
}
