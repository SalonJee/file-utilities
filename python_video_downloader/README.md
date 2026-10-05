# ytdl

A simple CLI tool written in Python that downloads audio (MP3) or video (MP4) from YouTube — supports individual videos and full playlists, with automatic duplicate skipping.

---

## Requirements

Make sure these are installed before anything:

```bash
# Python 3 (usually pre-installed on Linux)
python3 --version

# ffmpeg (for audio extraction and video merging)
sudo apt install ffmpeg           # Ubuntu / Debian
sudo dnf install ffmpeg           # Fedora
sudo pacman -S ffmpeg             # Arch

# yt-dlp (the download engine)
pip install yt-dlp
```

---

## Install Globally (Recommended)

Install once, use from anywhere.

### Step 1: Create a dedicated virtual environment

Since modern Linux distros block system-wide pip installs ([PEP 668](https://peps.python.org/pep-0668/)), we create a small venv just for this tool:

```bash
# Create the venv
python3 -m venv ~/.local/share/ytdl-env

# Install yt-dlp inside it
~/.local/share/ytdl-env/bin/pip install yt-dlp
```

### Step 2: Copy the script to your local bin

```bash
# Create the local bin folder if it doesn't exist
mkdir -p ~/.local/bin

# Copy the script over
cp main.py ~/.local/bin/ytdl

# Make it executable
chmod +x ~/.local/bin/ytdl
```

Make sure `~/.local/bin/` is in your system's PATH (it usually is on Linux).

That's it! The tool is now available system-wide as `ytdl`.

---

## Usage

```bash
ytdl
```

You'll be prompted to enter a URL. Two modes are available:

### 🎵 Audio Mode (default)

Just paste the URL — downloads audio as **MP3 (192 kbps)**:

```
Enter the YouTube Video or Playlist URL (prefix /vdo for video): https://www.youtube.com/watch?v=dQw4w9WgXcQ
🎵 Audio mode
```

### 🎬 Video Mode

Prefix the URL with `/vdo` — downloads the full **video as MP4**:

```
Enter the YouTube Video or Playlist URL (prefix /vdo for video): /vdo https://www.youtube.com/watch?v=dQw4w9WgXcQ
🎬 Video mode
```

### Examples

```bash
# Run interactively
ytdl

# Download audio from a playlist
# (paste the playlist URL when prompted)

# Download video
# (type /vdo before pasting the URL)
```

### Output

Files are saved in the current directory, organized into folders:

```
# Audio mode
Single Songs/
  Never Gonna Give You Up.mp3

# Video mode
Single Videos/
  Never Gonna Give You Up.mp4

# Playlists get their own folder
My Playlist Name/
  Song 1.mp3
  Song 2.mp3
```

---

## Features

| Feature | Details |
|---|---|
| **Audio download** | Best audio → MP3 at 192 kbps |
| **Video download** | Best quality MP4 (prefix `/vdo`) |
| **Playlist support** | ✅ Downloads all videos in a playlist |
| **Duplicate skipping** | Tracks downloads in `downloaded_songs.txt` |
| **Error handling** | Skips deleted/private videos automatically |

---

## Updating After Code Changes

If you edit `main.py` and want to apply the changes:

```bash
cp main.py ~/.local/bin/ytdl
```

---

## How It Works

1. Prompts for a YouTube URL (video or playlist)
2. If prefixed with `/vdo`, downloads video as MP4; otherwise extracts audio as MP3
3. Uses `yt-dlp` under the hood to handle downloading
4. Saves a record of downloaded videos in `downloaded_songs.txt` to avoid re-downloading
5. Organizes files into folders named after the playlist (or `Single Songs` / `Single Videos` for individual URLs)
