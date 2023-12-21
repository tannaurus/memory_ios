//
//  Stories.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI

struct StoryList: View {
    var stories: [Story]
    
    struct Card: View {
        let story: Story
        
        func viewStory() {
            print("Viewing", story.title)
        }
    
        var body: some View {
            VStack {
                HStack {
                    Text("11/23").font(.system(size: 10))
                    Spacer()
                    Text(story.title)
                    Spacer()
                }
                Image(story.preview)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .clipShape(
                            RoundedRectangle(cornerSize: CGSize(width: 10, height: 10))
                    )
            }
        }
    }
    
    var body: some View {
        NavigationSplitView {
            List(stories) { story in
                NavigationLink {
                    StoryDetails(story: story)
                } label: {
                    Card(story: story)
                }
            }
            .listStyle(.plain)
            .navigationTitle("Your Story")
        } detail: {
            Text("Select a Story")
        }
    }
}

#Preview {
    StoryList(stories: mock_stories)
}
