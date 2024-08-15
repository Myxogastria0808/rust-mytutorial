use std::{os::windows::thread, time::Duration};

fn main() {
    {
        use std::thread;

        //*シンクスレッドの作成*//
        //*thread::spawn(クロージャ)*//
        {
            println!("\n");
            println!("ただマルチスレッドにしただけ");
            //***作成したスレッドの実行が終わらない可能性がある。***//
            //JoinHandleは、そのjoinメソッドを呼び出したときにスレッドの終了を待つ所有された値

            thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned threads!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }

        {
            println!("\n");
            println!("JionHandleのパターン1");
            //*全スレッドが終了することを保証するために*//
            //* thread::spawnの戻り値がJoinHandleであり、その戻り値を保存することで解決する
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
            for i in 1..5 {
                println!("hi number {} from the main threads!", i);
                thread::sleep(Duration::from_millis(1));
            }

            //作成したスレッドが、実行終了するまでjoinメソッドは、待機する
            handle.join().expect("Error!");
        }
        {
            println!("\n");
            println!("JionHandleのパターン2");
            //以下のようにすると、すべてのhandleのメソッドが実行された後にmainのスレッドが実行される
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });

            //作成したスレッドが、実行終了するまでjoinメソッドは、待機する
            handle.join().expect("Error!");

            for i in 1..5 {
                println!("hi number {} from the main threads!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
        {
            //*別のスレッドに値を渡す*//
            //*渡したい値が存在することを保証するために、所有権クロージャにmoveする必要がある *//
            //*そうすることで、実行可能になる (moveできない時点で、コンパイルエラーになるため、大丈夫ということ)*//
            let v: Vec<i32> = vec![1, 2, 3];

            let handle = thread::spawn(move || {
                println!("here is a vector: {:?}", v);
            });

            handle.join().unwrap();
        }
    }

    {
        use std::sync::mpsc;
        //*Mutiple Producer, Single Consumer*//
        use std::thread;
        //***チャンネル***//
        //プログラミングにおけるチャンネルは、
        //2分割できます: 転送機と受信機です。
        //転送機はアヒルのおもちゃを川に置く上流になり、
        //受信機は、アヒルのおもちゃが行き着く下流になります。
        //コードのある箇所が送信したいデータとともに転送機のメソッドを呼び出し、
        //別の部分がメッセージが到着していないか受信側を調べます。
        //転送機と受信機のどちらかがドロップされると、
        //チャンネルは閉じられたと言います。
        {
            println!("\n");
            println!("チャンネルの作成");
            //*チャンネルの作成
            //*(送信機, 受信機) = mpsc::chnnel() *//
            let (tx, rx) = mpsc::channel();

            //送信機の所有権をmoveする
            thread::spawn(move || {
                let val: String = String::from("hi by recv method");
                //sendメソッドで送信できる。sendメソッドは、Result<(), SendErr<T>>型を返す
                tx.send(val).expect("送信エラー");
            });

            //*recvメソッド *//
            //メインスレッドの実行をブロックし、
            //値がチャンネルを流れてくるまで待機します。
            //一旦値が送信されたら、recvはそれをResult<T, E>に含んで返します。
            //*recvメソッドで受信できる。recvメソッドは、Result<T, RecvErr>型を返す
            let received: String = rx.recv().expect("受信エラー");
            println!("Get: {}!", received);

            //*try_recメソッド *//
            //メインスレッドの実行をブロックせず、
            //代わりに即座にResult<T, E>を返します:
            //メッセージがあったら、それを含むOk値、
            //今回は何もメッセージがなければ、Err値です。
        }

        {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).expect("送信エラー");
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for recied in rx {
                println!("Get: {}", recied);
            }
        }

        {
            println!("\n");
            println!("MPSCの難しさ");
            let (tx, rx) = mpsc::channel();

            let tx1: mpsc::Sender<String> = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let vals: Vec<String> = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];
                for val in vals {
                    tx1.send(val).expect("送信エラー");
                    thread::sleep(Duration::from_secs(1));
                }
            });

            thread::spawn(move || {
                let vals: Vec<String> = vec![
                    String::from("more"),
                    String::from("message"),
                    String::from("for"),
                    String::from("you"),
                ];
                for val in vals {
                    tx.send(val).expect("送信エラー");
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Get: {}", received);
            }
        }
    }
    {
        //***Mutex<T>の使い方***//
        //*複数のスレッド間で状態を共有するために使う*//
        //ミューテックスを現実世界の物で例えるなら、
        //マイクが1つしかない会議のパネルディスカッションを思い浮かべてください。
        //パネリストが発言できる前に、マイクを使用したいと申し出たり、
        //通知しなければなりません。マイクを受け取ったら、 話したいだけ話し、
        // /それから次に発言を申し出たパネリストにマイクを手渡します。
        //パネリストが発言し終わった時に、 マイクを手渡すのを忘れていたら、
        //誰も他の人は発言できません。共有されているマイクの管理がうまくいかなければ、
        // パネルは予定通りに機能しないでしょう！
        use std::sync::Mutex;

        {
            println!("\n");
            println!("MシングルスレッドのMutex");
            //*Mutextの生成
            let m: Mutex<i32> = Mutex::new(5);
            println!("Mutex<i32>: {:?}", m);
            {
                //*lockメソッドを使用してロックを獲得する。
                //*現在のスレッドをブロックするので、 ロックを得られる順番が来るまで何も作業はできません。
                let mut num: std::sync::MutexGuard<'_, i32> = m.lock().expect("Lockに失敗しました");
                //MutexGardが、Derefを実装しているので、内部のデータをnumに格納していることになる

                *num = 6;
                println!("Mutex<i32>: {:?}", m);
            }
            //MutexGuardが、LockをするDropを実装しているので、勝手にロックされる
            println!("Mutex<i32>: {:?}", m);
        }

        {
            use std::sync::Arc;
            use std::thread;
            //***複数の所有権を持たせる必要がある。***//
            //*マルチスレッドのRC<T>に当たるのが、Arc<T>。 *//
            println!("\n");
            println!("マルチスレッドのMutex");

            let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
            let mut handles: Vec<thread::JoinHandle<()>> = vec![];

            {
                let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
                let handle: thread::JoinHandle<()> = thread::spawn(move || {
                    //中身の参照を可変で受け取っている
                    let mut num: std::sync::MutexGuard<'_, i32> = counter.lock().expect("Error!");
                    *num += 1;
                });
                //ここで、handleは、handlesの要素としてmoveされる
                handles.push(handle);
            }

            {
                let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
                let handle: thread::JoinHandle<()> = thread::spawn(move || {
                    //中身の参照を可変で受け取っている
                    let mut num: std::sync::MutexGuard<'_, i32> = counter.lock().expect("Error!");
                    *num += 1;
                });
                //ここで、handleは、handlesの要素としてmoveされる
                handles.push(handle);
            }

            //*handleは、handlesにmoveされているため、以下のようにして、スレッドの終了を宣言する
            for handle in handles {
                handle.join().expect("Error");
            }

            //中身の参照を可変で受け取って、参照外しをすることで出力可能にしている
            println!("handles: {}", *counter.lock().expect("Error"));
        }
    }
}
