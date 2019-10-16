<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Busqueda_BarrioPrivado_CO</name>
   <tag></tag>
   <elementGuidId>f54a054b-f310-45c8-acf4-0277cb8e329b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:pred=&quot;http://predictivesearch.soap2.cartoonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;pred:predictiveAddressSearch>
         &lt;!--Optional:-->
         &lt;addressPredictiveNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddressPredictive>
               &lt;!--Optional:-->
               &lt;level1>AR&lt;/level1>
               &lt;!--Optional:-->
               &lt;stringSearch>la lomada del pilar&lt;/stringSearch>
            &lt;/xAddressPredictive>
         &lt;/addressPredictiveNormalize>
      &lt;/pred:predictiveAddressSearch>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>predictiveAddressSearch</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PredictiveSearch_ARG2</defaultValue>
      <description></description>
      <id>2866abb5-d641-4892-a76f-3c0b0b2ba2a5</id>
      <masked>false</masked>
      <name>PredictiveSearch_ARG2</name>
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


</verificationScript>
   <wsdlAddress>${PredictiveSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
