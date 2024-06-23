export interface EntrySettings {
    uuid: string;
    title: string;
    content: ContentPiece[]
    goals: Goal[]
    description: string
}
export interface Goal {
    completed: boolean
    title: string
}
export interface ContentPiece {
    type: "image" | "text" | "video" | "audio"
    path: string | undefined
    text: string | undefined
    craetionDate: number
}