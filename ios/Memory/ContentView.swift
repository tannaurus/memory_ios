//
//  ContentView.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

struct ContentView: View {
    var stories: [Story]
    var prompts: [Prompt]
    var user: User
    
    var body: some View {
        TabView {
            Home(stories: stories)
                .tabItem {
                    Image(systemName: "house.fill")
                    Text("Home")
                }
            CreateStory(prompts: prompts)
                .tabItem {
                    Image(systemName: "plus")
                    Text("Create")
                }
            Profile(user: user)
                .tabItem {
                    Image(systemName: "person.fill")
                    Text("Profile")
                }
        }.buttonBorderShape(.roundedRectangle)
    }
}

#Preview {
    ContentView(stories: mock_stories, prompts: mock_prompts, user: mock_user)
}
