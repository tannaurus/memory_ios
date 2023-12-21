//
//  Profile.swift
//  Memory
//
//  Created by Tanner on 10/22/23.
//

import SwiftUI

struct Profile: View {
    var user: User
    
    struct Header: View {
        var body: some View {
            HStack {
                Image(systemName: "plus")
                Spacer()
                Image(systemName: "line.3.horizontal")
            }.padding()
        }
    }
    
    struct ProfileStackedText: View {
        var label: String
        var value: String
        var body: some View {
            VStack {
                Text(value).foregroundStyle(.white).fontWeight(/*@START_MENU_TOKEN@*/.bold/*@END_MENU_TOKEN@*/)
                Text(label).foregroundStyle(.gray).font(.system(size: 14))
            }
        }
    }

    
    struct FollowButton: View {
        func onFollow() {
            print("Following ✅")
        }
        
        var body: some View {
            Button(action: onFollow) {
                Text("Follow").foregroundStyle(.white).fontWeight(.medium).padding(.horizontal, 30).padding(.vertical, 2)
            }.buttonStyle(.borderedProminent).controlSize(.large).tint(.purple)
        }
    }
    
    struct MessageButton: View {
        func onMessage() {
            print("Messaged ✅")
        }
        
        var body: some View {
            Button(action: onMessage) {
                Text("Message").foregroundStyle(.white).fontWeight(.medium).padding(.horizontal, 30).padding(.vertical, 2)
            }.buttonStyle(.bordered).controlSize(.large).tint(.white)
        }
    }
    
    var body: some View {
        VStack() {
            Header()
            VStack {
                HStack {
                    Spacer()
                    ProfileStackedText(label: "Followers", value: String(user.followers))
                    Spacer()
                    Image(user.picture)
                        .resizable()
                        .frame(width: 100, height: 100)
                        .clipShape(
                            RoundedRectangle(cornerSize: CGSize(width: 20, height: 20)))
                    Spacer()
                    ProfileStackedText(label: "Following", value: String(user.following))
                    Spacer()
                }.padding(10)
                VStack {
                    Text(user.name).foregroundStyle(.white).fontWeight(/*@START_MENU_TOKEN@*/.bold/*@END_MENU_TOKEN@*/).font(.system(size:26))
                    Text(user.bio).foregroundStyle(.gray).font(.system(size:15))
                    HStack {
                        FollowButton()
                        MessageButton()
                    }.padding(.vertical, 10)
                }
                Spacer()
            }
            .background(Color(red: 30/255, green: 33/255, blue: 41/255))
            .clipShape(UnevenRoundedRectangle(cornerRadii: RectangleCornerRadii(topLeading: 20, topTrailing: 20)))
            Spacer()
        }
    }
}

#Preview {
    Profile(user: mock_user)
}
