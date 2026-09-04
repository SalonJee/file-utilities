import yt_dlp

url = input("Enter the YouTube Video or Playlist URL: ")

# We'll store the 'memory' of downloaded songs in this file
archive_file = 'downloaded_songs.txt'

ydl_opts = {
    'format': 'bestaudio/best',
    'noplaylist': False,
    # This is the magic line: it skips anything already in the archive
    'download_archive': archive_file, 
    'postprocessors': [{
        'key': 'FFmpegExtractAudio',
        'preferredcodec': 'mp3',
        'preferredquality': '192',
    }],
    # Saves into a folder named after the playlist (or "Single Songs" for individual videos)
    'outtmpl': '%(playlist_title|Single Songs)s/%(title)s.%(ext)s',
    'ignoreerrors': True,  # Skip songs that are deleted or private
}

try:
    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        ydl.download([url])
    print("\n✅ Sync complete! New songs downloaded, existing ones skipped.")
except Exception as e:
    print(f"\n❌ An error occurred: {e}")