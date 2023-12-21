//
//  ContentView.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

struct ContentView: View {
   
    var body: some View {
        Profile(profile_picture: Image("profile"), follower_count: 432, following_count: 341, user_name: "Tanner Gill", user_bio: "Developer @ 1Password")
        FooterNavigator()
    }
}

#Preview {
    ContentView()
}
