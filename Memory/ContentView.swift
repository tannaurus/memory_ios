//
//  ContentView.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

struct ContentView: View {
    var stories: [Story]
    var body: some View {
        TabView {
            Home(stories: stories)
                .tabItem {
                    Image(systemName: "house.fill")
                    Text("Home")
                }
            Profile(profile_picture: Image("profile"), follower_count: 432, following_count: 341, user_name: "Tanner Gill", user_bio: "Developer @ 1Password")
                .tabItem() {
                Image(systemName: "person.fill")
                Text("Profile")
            }
        }.buttonBorderShape(.roundedRectangle)
    }
}

#Preview {
    ContentView(stories: mock_stories)
}
