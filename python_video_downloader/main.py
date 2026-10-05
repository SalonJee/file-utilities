#!/home/salon-timsina/.local/share/ytdl-env/bin/python3

import yt_dlp

user_input = input("Enter the YouTube Video or Playlist URL (prefix /vdo for video): ").strip()

# Check if user wants video instead of audio
video_mode = user_input.startswith("/vdo")
if video_mode:
    url = user_input[4:].strip()  # Remove "/vdo" prefix
    print("🎬 Video mode")
else:
    url = user_input
    print("🎵 Audio mode")

if not url:
    print("❌ No URL provided.")
    exit(1)

# We'll store the 'memory' of downloaded songs in this file
archive_file = 'downloaded_songs.txt'

if video_mode:
    ydl_opts = {
        'format': 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best',
        'noplaylist': False,
        'download_archive': archive_file,
        'merge_output_format': 'mp4',
        'outtmpl': '%(playlist_title|Single Videos)s/%(title)s.%(ext)s',
        'ignoreerrors': True,
    }
else:
    ydl_opts = {
        'format': 'bestaudio/best',
        'noplaylist': False,
        'download_archive': archive_file,
        'postprocessors': [{
            'key': 'FFmpegExtractAudio',
            'preferredcodec': 'mp3',
            'preferredquality': '192',
        }],
        'outtmpl': '%(playlist_title|Single Songs)s/%(title)s.%(ext)s',
        'ignoreerrors': True,
    }

try:
    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        ydl.download([url])
    print("\n✅ Sync complete! New downloads finished, existing ones skipped.")
except Exception as e:
    print(f"\n❌ An error occurred: {e}")
    print(f"\n❌ An error occurred: {e}")