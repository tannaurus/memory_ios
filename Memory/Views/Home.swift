//
//  Home.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI

struct Home: View {
    var body: some View {
        VStack {
            StoryList(stories: [Story(title: "Spring", preview: "user_1")])
            Spacer()
        }
    }
}

#Preview {
    Home()
}
