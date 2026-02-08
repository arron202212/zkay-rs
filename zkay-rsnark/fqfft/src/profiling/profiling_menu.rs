//  Implementation of functions for profiling menu.

// /* Level 2: Profile */
// pub fn  profile()
// {
//   std::cout << "\n\033[1;32mChoose Profile Type:\033[0m\n";
//   std::cout << "\033[1;32m(i.e. To Select All, input '1 2 3')\033[0m\n\n";
//   std::cout << "\033[1;36m 1: Runtime\033[0m\n";
//   std::cout << "\033[1;36m 2: Memory\033[0m\n";
//   std::cout << "\033[1;36m 3: Operators\033[0m\n";
//   std::cout << "\033[1;36m 4: (Back)\033[0m\n\n";

//   int valid_input = 0;
//   String profile_type;
//   do {
//     std::cout << "> ";
//     getline(std::cin, profile_type);

//     /* Check Input Validity */
//     int n;
//     valid_input = 1;
//     std::stringstream stream(profile_type);
//     while (stream >> n)
//     {
//       if n < 1 || n > 4 valid_input = 0;
//       if n == 4 valid_input = 2;
//     }
//   } while (!valid_input);

//   /* Level 3: Choose Domain Type */
//   if valid_input == 1
//   {
//     std::cout << "\n\033[1;32mChoose Domain Type:\033[0m\n\n";
//     std::cout << "\033[1;36m 1: All\033[0m\n";
//     std::cout << "\033[1;36m 2: Radix-2\033[0m\n";
//     std::cout << "\033[1;36m 3: Arithmetic & Geometric\033[0m\n";
//     std::cout << "\033[1;36m 4: (Back)\033[0m\n\n";

//     valid_input = 0;
//     String domain;
//     do {
//       std::cout << "> ";
//       getline(std::cin, domain);

//       int n;
//       valid_input = 1;
//       std::stringstream stream(domain);
//       while (stream >> n)
//       {
//         if n < 1 || n > 4 valid_input = 0;
//         if n == 4 valid_input = 2;
//       }
//     } while (!valid_input);
//     int domain_type = atoi(domain);

//     /* Level 4: Choose Domain Sizes */
//     if valid_input == 1
//     {
//       /* Built-in domain size choices */
//       Vec<String> domains(3);
//       domains[0] = "32768 65536 131072 262144";
//       domains[1] = "131072 262144 524288 1048576";

//       std::cout << "\n\033[1;32mChoose Domain Sizes:\033[0m\n\n";
//       std::cout << "\033[1;36m 1: Small - [" << domains[0] << "]\033[0m\n";
//       std::cout << "\033[1;36m 2: Large - [" << domains[1] << "]\033[0m\n";
//       std::cout << "\033[1;36m 3: Custom\033[0m\n";
//       std::cout << "\033[1;36m 4: (Back)\033[0m\n\n";

//       do {
//         std::cout << "> ";
//         getline(std::cin, domain);
//       } while (strcmp(domain, "1")
//             && strcmp(domain, "2")
//             && strcmp(domain, "3")
//             && strcmp(domain, "4"));
//       int domain_choice = atoi(domain) - 1;

//       if domain_choice >= 0 && domain_choice < 3
//       {
//         /* Level 5: Custom Domain Choice */
//         if domain_choice == 2
//         {
//           std::cout << "\n\033[1;32mEnter Custom Domain:\033[0m\n";
//           std::cout << "\033[1;32m(i.e. \"32768 65536 131072 262144\")\033[0m\n\n";
//           String custom_dom;
//           bool custom_input = 0;
//           do {
//             std::cout << "> ";
//             getline(std::cin, custom_dom);

//             /* Check Input Validity */
//             if strcmp(custom_dom, "") && strcmp(custom_dom, " ")
//             {
//               std::stringstream stream(custom_dom);
//               custom_input = 1;
//               int n;
//               while (stream >> n) if n < 1 custom_input = 0;
//             }
//           } while (!custom_input);
//           domains[2] = custom_dom;
//         }

//         /* Get Current Timestamp */
//         time_t rawtime;
//         time(&rawtime);
//         struct tm* timeinfo = localtime(&rawtime);
//         char buffer[40];
//         strftime(buffer, 40, "%m-%d_%I:%M", timeinfo);
//         String datetime(buffer);

//         /* Perform Profiling */
//         std::cout << "\n\033[1;32mStarting Profiling:\033[0m\n";
// // #ifdef PROF_DOUBLE
//         print!("Profiling with Double\n");
// // #else
//         print!("Profiling with Fr<edwards_pp>\n");
// //#endif
//         for threads in 0..4
//         {
//           for key in 0..4 /* Change key to 5 for arithmetic domain */
//           {
//             if key > 2 && domain_type == 2 continue;
//             if key < 3 && domain_type == 3 continue;
//             if (system(("./profiler "
//                         + std::to_string(key) + " "
//                         + std::to_string(1u << threads) + " "
//                         + datetime + " "
//                         + "\"" + profile_type + "\" "
//                         + "\"" + domains[domain_choice] + "\"")))
//               print!("\n error: profiling\n");
//           }
//         }
//         std::cout << "\n\033[1;32mDone Profiling\033[0m\n";
//       }
//     }
//   }
// }

// /* Level 2: Plot */
// pub fn  plot()
// {
//   std::cout << "\n\033[1;32mChoose Profile:\033[0m\n\n";
//   std::cout << "\033[1;36m 1: Operators\033[0m\n";
//   std::cout << "\033[1;36m 2: Runtime\033[0m\n";
//   std::cout << "\033[1;36m 3: Memory\033[0m\n";
//   std::cout << "\033[1;36m 4: (Back)\033[0m\n\n";

//   int type;
//   String res;
//   do {
//     std::cout << "> ";
//     getline(std::cin, res);
//     type = atoi(res);
//   } while (type < 1 || type > 4);

//   /* If not (back) option */
//   if type > 0 && type < 4
//   {
//     /* Source Directory */
//     Vec< String > path (3);
//     path[0] = "libfqfft/profiling/logs/operators/";
//     path[1] = "libfqfft/profiling/logs/runtime/";
//     path[2] = "libfqfft/profiling/logs/memory/";

//     Vec< String > gnufile (3);
//     gnufile[0] = "libfqfft/profiling/plot/operators_plot.gp";
//     gnufile[1] = "libfqfft/profiling/plot/runtime_plot.gp";
//     gnufile[2] = "libfqfft/profiling/plot/memory_plot.gp";

//     /* Level 3: File to Plot */
//     DIR *dir;
//     if (dir = opendir(path[type - 1])) != NULL
//     {
//       std::cout << "\n\033[1;32mSelect File to Plot:\033[0m\n\n";

//       int count = 1;
//       struct dirent *pDirent;
//       Vec<char*> files;
//       while ((pDirent = readdir(dir)) != NULL)
//       {
//         if pDirent->d_name[0] != '.'
//         {
//           files.push_back(pDirent->d_name);
//           std::cout << "\033[1;36m "<< count++ << ": " << pDirent->d_name << "\033[0m\n";
//         }
//       }
//       std::cout << "\033[1;36m " << count << ": (Back)\033[0m\n\n";
//       closedir(dir);

//       /* Select File to Plot */
//       int file_number;
//       String file;
//       loop {
//         std::cout << "> ";
//         getline(std::cin, file);
//         file_number = atoi(file);
//         if file_number > 0 && file_number <=  files.len() + 1{
//             break
//         }
//       }

//       if file_number < count
//       {
//         /* Log Files Path */
//         String log_path = path[type - 1] + files[file_number - 1];
//         /* System Call */
//         String cmd = "gnuplot -e \"input_directory=\'" + log_path + "\'\" " + gnufile[type - 1];
//         if system(cmd) == 0 {print!("Plotted in {}\n", log_path);}
//       }
//     }
//   }
// }

// /* Level 1: Choose to Profile or Plot */
// int main()
// {
//   bool resume = 1;
//   while (resume)
//   {
//     std::cout << "\n\033[1;32mChoose:\033[0m\n\n";
//     std::cout << "\033[1;36m 1: Profile\033[0m\n";
//     std::cout << "\033[1;36m 2: Plot\033[0m\n";
//     std::cout << "\033[1;36m 3: Exit\033[0m\n\n";

//     String res;
//     do {
//       std::cout << "> ";
//       getline(std::cin, res);
//     } while (strcmp(res, "1")
//           && strcmp(res, "2")
//           && strcmp(res, "3"));

//     if strcmp(res, "1") == 0) profile(;
//     else if strcmp(res, "2") == 0) plot(;
//     else if strcmp(res, "3") == 0 resume = 0;
//   }

//   return 0;
// }
