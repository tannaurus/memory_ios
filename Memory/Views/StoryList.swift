//
//  Stories.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI


struct Story: Codable, Identifiable {
    let id = UUID()
    
    var title: String;
    var preview: String;
    
    private enum CodingKeys: String, CodingKey {
        case title
        case preview
    }
}


struct StoryList: View {
    let stories: [Story]
    
    struct Card: View {
        let story: Story
        
        func viewStory() {
            print("Viewing", story.title)
        }
        
        var body: some View {
            VStack {
                Button(action: viewStory) {
                    Image(story.preview)
                            .resizable()
                            .clipShape(
                                    RoundedRectangle(cornerSize: CGSize(width: 10, height: 10))
                            )

                }
                Text(story.title).font(.system(size: 10))
            }
        }
    }
    
    var body: some View {
        List(stories) {
            Card(story: $0)
        }
    }
}

#Preview {
    StoryList(stories: [Story(title: "Spring", preview: "user_1")])
}
