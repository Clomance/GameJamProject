use cat_engine::audio::{
    AudioSettings,
    Audio,
    cpal,
    cpal::traits::HostTrait,
    cpal::traits::DeviceTrait,
    AudioWrapper,
};


fn main(){
    let settings=AudioSettings::new();
    let host=cpal::default_host();
    let audio=Audio::new(host,|host|{
        host.default_output_device().unwrap()
    },
                         |device|{
         device.default_output_format().unwrap()
     },
     settings
    ).unwrap();

    let mut audio_wrapper=AudioWrapper::new(audio);

    let input=std::io::stdin();
    let mut line=String::new();
    while let Ok(_)=input.read_line(&mut line){
        let mut parts=line.trim().split_ascii_whitespace();
        if let Some(command)=parts.next(){
            match command{
                "help"=>{
                    println!("load [track_path] [track_name]");
                    println!("remove [track_name]");
                    println!("play +| [track_name]");
                    println!("set --g volume [value] | set --t [track_name] volume [value]");
                    println!("pause +| [track_name]");
                    println!("stop +| [track_name]");
                    println!("close");
                }

                "load"=>if let Some(path)=parts.next(){
                    if let Some(name)=parts.next(){
                        println!("Loading {}...",path);
                        if audio_wrapper.load_track(path,name.to_string()){
                            println!("Loaded");
                        }
                        else{
                            println!("Error");
                        }
                    }
                }

                "remove"=>if let Some(option)=parts.next(){
                    match option{
                        "--s"=>{
                            if let Some(_track)=parts.next(){
                                //let index:usize=track.parse().unwrap();
                                //audio_wrapper.remove_mono_track(index);
                            }
                        }
                        name=>audio_wrapper.remove_track(name).unwrap()
                    }
                }

                "play"=>if let Some(option)=parts.next(){
                    match option{
                        // single channel
                        "--s"=>{
                            if let Some(track)=parts.next(){
                                let _index:usize=track.parse().unwrap();
                                //audio_wrapper.unpause_mono_track(index);

                                //     // Номер канала
                                //     if let Some(channel)=parts.next(){
                                //         let channel:usize=channel.parse().unwrap();

                                //         if let Some(repeats)=parts.next(){
                                //             let repeats=repeats.parse().unwrap();
                                //             audio.play_track(track,&[channel],repeats).unwrap();
                                //         }
                                //         else{
                                //             audio.play_track(track,&[channel],1).unwrap();
                                //         }
                                //     }
                            }
                        }

                        _=>{}
                        //name=>audio_wrapper.play_track(name).unwrap()
                    }
                }
                else{
                    // Убрать паузу с потока
                    audio_wrapper.play();
                }

                "pause"=>if let Some(option)=parts.next(){
                    match option{
                        // single channel
                        "--s"=>{
                            if let Some(track)=parts.next(){
                                let _index:usize=track.parse().unwrap();
                                //audio_wrapper.pause_mono_track(index).unwrap();
                            }
                        }
                        _=>{}
                        //name=>audio_wrapper.pause_track(name).unwrap(),
                    }
                }
                else{
                    audio_wrapper.pause().unwrap()
                }

                "set"=>if let Some(option)=parts.next(){
                    match option{
                        "--g"=>if let Some(parametr)=parts.next(){
                            if let Some(value)=parts.next(){
                                match parametr{
                                    "volume"=>{
                                        let volume=value.parse().unwrap();
                                        audio_wrapper.set_general_volume(volume);
                                    }
                                    _=>{}
                                }
                            }
                        }
                        "--t"=>if let Some(_name)=parts.next(){
                            if let Some(parametr)=parts.next(){
                                if let Some(value)=parts.next(){
                                    match parametr{
                                        "volume"=>{
                                            let _volume:usize=value.parse().unwrap();
                                            //audio_wrapper.set_track_volume(track,volume).unwrap()
                                        }
                                        _=>{}
                                    }
                                }
                            }
                        }
                        _=>{}
                    }
                }

                "stop"=>if let Some(_name)=parts.next(){
                    //audio_wrapper.stop_track(name).unwrap()
                }
                else{
                    audio_wrapper.clear_playlist().unwrap()
                }

                "close"=>break,
                _=>{}
            }
        }

        line.clear();
    }
}