{ 
      networking.hostName = "whim";
    
      networking.bonds.bond0 = {
        driverOptions.mode = "balance-tlb";
        interfaces = [
          "enp0s20f0" "enp0s20f1"
        ];
      };
    
      networking.interfaces.bond0 = {
        useDHCP = true;

        ip4 = [
          
          {
            address = "147.75.73.83";
            prefixLength = 31;
          }
    

          {
            address = "10.100.130.131";
            prefixLength = 31;
          }
    
        ];

        ip6 = [
          
          {
            address = "2604:1380:5:6400::3";
            prefixLength = 127;
          }
    
        ];
      };
    
      users.users.root.openssh.authorizedKeys.keys = [
        "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCptTOyI/zRkHglGf6tfXLFDM39xNEztGjNoDmWAqU4//rSrLA2OGs7nTpE9ML9IIG9BDozyQXoQ63PO9Qa6OQyt4fdO/SRucqwOVfMTSmleHzl38Enb4FjiU3qPUrJxgshM1fJndHl3YQf34eoIHKyBpQq5QIRV/NZRA9122Jw42QQc15xKQeEjEehtY11zE89CYpnUYUoubd3SkqxlGrtqURZ4mxRLxgJGotvNL7W6mNm0SL0tne4fChcWI328ko0hwDC7+uGrVdM0dnBkL1CMDaxaepgvO1vhT99rn/awyueb3zCiTBBkUaQK/ZLp/cD7Brh72eGy3/RdGeK5oftycykCgJup7zRce2FBjXf7j4UKkoTjckEmxj7OlISpZitOv2tulWQqsykC/yQkUsiOioBkHOMSwLadHHP6gH0D3d59GkMNcsL8+uNVHtrP00dcqCTwQEHl5XKYeCaUOIcW3izcXKNgigUryoElnbjTlVJ5r24a5qb6jIBDPLeoJWftRXtSdIVSsT/6grEpjNVPxHNdu4QgyBvnC8i7AxHEeNIw7+mvZuiIyTZl2pK373FoAjyKs/p/sGqWeFLJy+NSpoxGyTI8WAcSbEuMSn0oOnDynKsZve6ZDHlez7e1qRkiEJH2ucHOsGUvnlQ5xC1es5XbuzFbP6zPaqmpSNLSQ== durandal"
    
      ];
     }
