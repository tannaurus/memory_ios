//
//  StoryDetails.swift
//  Memory
//
//  Created by Tanner on 11/24/23.
//

import SwiftUI

struct StoryDetails: View {
    var story: Story
    
    struct StoryActions: View {
        func share() {
            print("share")
        }
        
        func edit() {
            print("edit")
        }
        
        func delete() {
            print("delete")
        }
        
        var body: some View {
            Menu("Actions") {
                Button("Share", action: share)
                Button("Edit", action: edit)
                Button("Delete", action: delete)
            }
        }
    }
    
    var body: some View {
        VStack {
            HStack {
                Text("11/23").font(.system(size: 10))
                Spacer()
                Text(story.title)
                Spacer()
                StoryActions()
            }.padding(20)
            List(story.images) { image in
                VStack {
                    Image(image.image)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .clipShape(
                            RoundedRectangle(cornerSize: CGSize(width: 10, height: 10))
                        )
                    HStack {
                        Text(image.description)
                        Spacer()
                    }
                }
            }.listStyle(.plain)
        }
    }
}

#Preview {
    StoryDetails(story: mock_stories[1])
}
