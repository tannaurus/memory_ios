//
//  Stories.swift
//  Memory
//
//  Created by Tanner on 10/29/23.
//

import SwiftUI

struct StoryList: View {
    struct Story: View {
        let userName: String
        let userIcon: Image
        
        func viewStory() {
            print("Viewing", userName)
        }
        
        var body: some View {
            VStack {
                Button(action: viewStory) {
                    userIcon
                            .resizable()
                            .frame(width: 75, height: 75)
                            .clipShape(
                                    RoundedRectangle(cornerSize: CGSize(width: 20, height: 20))
                            )

                }
                Text(userName).font(.system(size: 10))
            }
        }
    }
    
    struct YourStory: View {
        func viewStory() {
            print("Viewing your story")
        }
        
        var body: some View {
            VStack {
                Button(action: viewStory) {
                    Image(systemName: "plus")
                            .resizable()
                            .frame(width: 75, height: 75)
                            .clipShape(
                                    RoundedRectangle(cornerSize: CGSize(width: 20, height: 20))
                            )

                }
                Text("Your Story").font(.system(size: 10))
            }
        }
    }
    
    var body: some View {
        ScrollView([.horizontal], showsIndicators: false) {
            HStack {
                YourStory()
                Story(userName: "soft_bloom", userIcon: Image("user_1"))
                Story(userName: "soft_bloom", userIcon: Image("user_1"))
                Story(userName: "soft_bloom", userIcon: Image("user_1"))
                Story(userName: "soft_bloom", userIcon: Image("user_1"))
            }
        }.padding(.leading, 20)
    }
}

#Preview {
    StoryList()
}
