//
//  Home.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI

struct Home: View {
    var stories: [Story]
    var body: some View {
        VStack {
            StoryList(stories: stories)
        }
    }
}

#Preview {
    Home(stories: mock_stories)
}
