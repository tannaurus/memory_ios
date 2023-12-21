//
//  Stories.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI
import Foundation

struct Story: Codable, Identifiable {
    let id = UUID()
    
    var title: String;
    var preview: String;
    var created_at: String;
    var updated_at: String;
    
    private enum CodingKeys: String, CodingKey {
        case title
        case preview
        case created_at
        case updated_at
    }
}


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
                Button(action: viewStory) {
                    Image(story.preview)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .clipShape(
                                RoundedRectangle(cornerSize: CGSize(width: 10, height: 10))
                        )

                }
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
    StoryList(stories: mock_stories)
}
