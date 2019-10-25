<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DZ</name>
   <tag></tag>
   <elementGuidId>be648027-bf40-428c-a094-fd21df9daf92</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>POSADAS&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>BARRIO 25 DE MAYO&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>3300&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>4d59abac-efa3-41b7-bd77-7640915b76d1</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


/* Verificacion Test Case */

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'MISIONES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'POSADAS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'BARRIO 25 DE MAYO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-27.368742')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-55.894215')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '3300')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '2')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>MISIONES&lt;/level2>&lt;level3>CAPITAL&lt;/level3>&lt;level4>POSADAS&lt;/level4>&lt;level5>25 DE MAYO&lt;/level5>&lt;street>Barrio 25 DE MAYO&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-27.392435&lt;/latitude>&lt;longitude>-55.902976&lt;/longitude>&lt;postalCode>3300&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>MISIONES&lt;/level2>&lt;level3>CAPITAL&lt;/level3>&lt;level4>POSADAS&lt;/level4>&lt;level5>1 DE MAYO&lt;/level5>&lt;street>Barrio 1 DE MAYO&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-27.362872&lt;/latitude>&lt;longitude>-55.925473&lt;/longitude>&lt;postalCode>3300&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-55.894215&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-27.368742&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>6&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-27.368742&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-55.894215&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>6&lt;/value>')

/**Verificacion WS unitaria
WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'MISIONES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'CAPITAL')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'POSADAS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'BARRIO 25 DE MAYO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-27.368742')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-55.894215')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '3300')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '2')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>MISIONES&lt;/level2>&lt;level3>CAPITAL&lt;/level3>&lt;level4>POSADAS&lt;/level4>&lt;level5>25 DE MAYO&lt;/level5>&lt;street>Barrio 25 DE MAYO&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-27.392435&lt;/latitude>&lt;longitude>-55.902976&lt;/longitude>&lt;postalCode>3300&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>MISIONES&lt;/level2>&lt;level3>CAPITAL&lt;/level3>&lt;level4>POSADAS&lt;/level4>&lt;level5>1 DE MAYO&lt;/level5>&lt;street>Barrio 1 DE MAYO&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-27.362872&lt;/latitude>&lt;longitude>-55.925473&lt;/longitude>&lt;postalCode>3300&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-55.894215&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-27.368742&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>6&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-27.368742&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-55.894215&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>6&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
